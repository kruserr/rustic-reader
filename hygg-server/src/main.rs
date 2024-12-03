use bytes::BufMut;
// use cli_text_reader_online::progress::Progress;
use futures::{StreamExt, TryStreamExt};
use tracing_subscriber::fmt::format::FmtSpan;
use std::convert::Infallible;
use uuid::Uuid;

use warp::{http::StatusCode, multipart::FormData, Filter, Rejection, Reply};
use serde_derive::{Deserialize, Serialize};
use polodb_core::{CollectionT, Database};
use polodb_core::bson::{Document, doc};
use chrono::{DateTime, Utc};

pub fn mkdir(
  input: &str,
) -> Result<String, Box<dyn std::error::Error + Send>> {
  let path = std::path::Path::new(input);
  std::fs::create_dir_all(path)
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;
  Ok("".into())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenedDTO {
  session_id: String,
  document_hash: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Opened {
  timestamp: DateTime<Utc>,

  session_id: String,
  document_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgressDTO {
  session_id: String,
  document_hash: String,
  offset: String,
  total_lines: String,
  percentage: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Progress {
  timestamp: DateTime<Utc>,

  session_id: String,
  document_hash: String,
  offset: String,
  total_lines: String,
  percentage: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_span_events(FmtSpan::CLOSE).init();

    let uploads_dir = "hygg-server-uploads";
    mkdir(uploads_dir);

    let db_path = "hygg-server-db";
    let db_collection_opened = "opened";
    let db_collection_progress = "progress";
    let db = std::sync::Arc::new(Database::open_file(db_path).unwrap());

    let route_post_upload = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(1024 * 1024 * 50))
        .and_then(move |x| upload(x, &uploads_dir))
    ;

    let route_get_download = warp::path("download")
      .and(warp::get())
      .and(warp::fs::dir(uploads_dir))
    ;

    let route_post_opened_db = db.clone();
    let route_post_opened = warp::path("opened")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(move |opened: OpenedDTO| {
            println!("{:?}", opened);

            // let txn = db.start_transaction().unwrap();
            // let collection = txn.collection::<Document>(db_collection_opened);

            let collection = route_post_opened_db.collection::<Opened>(db_collection_opened);

            collection.insert_one(Opened {
              timestamp: chrono::Utc::now(), 

              session_id: opened.session_id,
              document_hash: opened.document_hash,
            }).unwrap();

            warp::reply::json(&"3".to_owned())
        })
    ;

    let route_get_opened_db = db.clone();
    let route_get_opened = warp::path("opened")
        .and(warp::get())
        .and(warp::path::param::<String>())
        .map(move |document_hash: String| {
            // let txn = db.start_transaction().unwrap();
            // let collection = txn.collection::<Document>(db_collection_opened);

            let collection = route_get_opened_db.collection::<Opened>(db_collection_opened);

            let res = collection
              .find(doc! {
                  "document_hash": document_hash,
              })
              .sort(doc! {
                  "timestamp": -1,
              })
              .limit(1)
              .run().unwrap();

            for item in res {
                println!("{:?}", item);
            }

            warp::reply::json(&"2".to_owned())
        })
    ;

    let route_post_progress = warp::path("progress")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|progress: ProgressDTO| {
            println!("{:?}", progress);

            warp::reply::json(&progress)
        })
    ;

    let router = route_post_upload
      .or(route_get_download)
      .or(route_post_opened)
      .or(route_get_opened)
      .or(route_post_progress)
      // .recover(handle_rejection)
    ;

    warp::serve(router.with(warp::trace::request())).run(([0, 0, 0, 0], 3030)).await;
}

async fn upload(form: FormData, uploads_dir: &str) -> Result<impl Reply, Rejection> {
    let mut parts = form.into_stream();
    while let Some(Ok(p)) = parts.next().await {
        if p.name() == "file" {
            let content_type = p.content_type();
            let file_ending;

            println!("{content_type:?}");

            match content_type {
                Some(file_type) => match file_type {
                    "application/pdf" => {
                        file_ending = "pdf";
                    }
                    "application/epub+zip" => {
                        file_ending = "epub";
                    }
                    "text/html" => {
                        file_ending = "html";
                    }
                    v => {
                        eprintln!("invalid file type found: {}", v);
                        return Err(warp::reject::reject());
                    }
                },
                None => {
                    eprintln!("file type could not be determined");
                    return Err(warp::reject::reject());
                }
            }

            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("reading file error: {}", e);
                    warp::reject::reject()
                })?;

            let file_name = format!("./{uploads_dir}/{}.{}", Uuid::new_v4(), file_ending);
            tokio::fs::write(&file_name, value).await.map_err(|e| {
                eprint!("error writing file: {}", e);
                warp::reject::reject()
            })?;
            println!("created file: {}", file_name);
        }
    }

    Ok("success")
}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}
