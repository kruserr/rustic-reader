use bytes::BufMut;
use cli_text_reader_online::progress::generate_hash;
// use cli_text_reader_online::progress::Progress;
use futures::{StreamExt, TryFutureExt, TryStreamExt};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing_subscriber::fmt::format::{self, FmtSpan};
use std::convert::Infallible;
use std::sync::Arc;
use uuid::Uuid;

use warp::{http::StatusCode, multipart::FormData, Filter, Rejection, Reply};
use serde_derive::Deserialize;
use polodb_core::{ClientCursor, CollectionT, Database};
use polodb_core::bson::{doc, document, Document};
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
pub struct HyggDocument {
  timestamp: DateTime<Utc>,

  name: String,
  document_hash: String,
  path: String,
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
    let db_collection_hygg_document = "hygg_document";
    let db_collection_opened = "opened";
    let db_collection_progress = "progress";
    let db = std::sync::Arc::new(Database::open_file(db_path).unwrap());

    let route_post_upload_db = db.clone();
    let route_post_upload = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(1024 * 1024 * 50))
        .and(warp::path::param::<String>())
        .and_then(move |x, name| handler_upload(x, &uploads_dir, name, route_post_upload_db.clone(), &db_collection_hygg_document))
    ;

    let route_get_list_db = db.clone();
    let route_get_list = warp::path("list")
        .and(warp::get())
        .map(move || {
            let txn = route_get_list_db.start_transaction().unwrap();
            let collection = txn.collection::<HyggDocument>(db_collection_hygg_document);

            let documents = collection
              .find(doc! {
              })
              .run().unwrap();

            let mut res = vec![];
            for item in documents {
              if let Ok(x) = item {
                res.push(HyggDocument {
                  timestamp: x.timestamp,
                  name: x.name,
                  document_hash: x.document_hash,
                  path: x.path,
                });
              }
            }

            return warp::reply::json(&res);
        })
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

            let txn = route_post_opened_db.start_transaction().unwrap();
            let collection = txn.collection::<Opened>(db_collection_opened);

            collection.insert_one(Opened {
              timestamp: chrono::Utc::now(), 

              session_id: opened.session_id,
              document_hash: opened.document_hash,
            }).unwrap();

            txn.commit().unwrap();

            return warp::reply::json(&"200".to_owned());
        })
    ;

    let route_post_progress_db = db.clone();
    let route_post_progress = warp::path("progress")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(move |progress: ProgressDTO| {
            println!("{:?}", progress);

            let session = get_latest_by_document_hash::<Opened>(
              route_post_progress_db.clone(),
              db_collection_opened,
              &progress.document_hash
            );

            for item in session {
                if let Ok(x) = item {
                  if (x.session_id != progress.session_id) {
                    return warp::reply::json(&"400".to_owned());
                  }

                  let txn = route_post_progress_db.start_transaction().unwrap();
                  let collection = txn.collection::<Progress>(db_collection_progress);

                  collection.insert_one(Progress {
                    timestamp:chrono::Utc::now(),

                    session_id: progress.session_id,
                    document_hash: progress.document_hash,
                    offset: progress.offset,
                    total_lines: progress.total_lines,
                    percentage: progress.percentage,
                  }).unwrap();

                  txn.commit().unwrap();

                  return warp::reply::json(&"200".to_owned());
                }
            }

            return warp::reply::json(&"500".to_owned());
        })
    ;

    let route_get_progress_db = db.clone();
    let route_get_progress = warp::path("progress")
        .and(warp::get())
        .and(warp::path::param::<String>())
        .map(move |document_hash: String| {
            let progress = get_latest_by_document_hash::<Progress>(
              route_get_progress_db.clone(),
              db_collection_progress,
              &document_hash,
            );

            let mut res = vec![];
            for item in progress {
              if let Ok(x) = item {
                res.push(Progress {
                  timestamp: x.timestamp,
                  session_id: x.session_id,
                  document_hash: x.document_hash,
                  offset: x.offset,
                  total_lines: x.total_lines,
                  percentage: x.percentage,
                });
              }
            }

            return warp::reply::json(&res);
        })
    ;

    let router = route_post_upload
      .or(route_get_list)
      .or(route_get_download)
      .or(route_post_opened)
      .or(route_post_progress)
      .or(route_get_progress)
    ;

    warp::serve(router.with(warp::trace::request())).run(([0, 0, 0, 0], 3030)).await;
}

fn get_latest_by_document_hash<T: Serialize + Send + Sync + DeserializeOwned>(db: Arc<Database>, db_collection_name: &str, document_hash: &str) -> ClientCursor<T> {
    let txn = db.start_transaction().unwrap();
    let collection = txn.collection::<T>(db_collection_name);

    let res = collection
      .find(doc! {
          "document_hash": document_hash,
      })
      .sort(doc! {
          "timestamp": -1,
      })
      .limit(1)
      .run().unwrap();

    return res;
}

async fn handler_upload(form: FormData, uploads_dir: &str, name: String, db: Arc<Database>, db_collection_name: &str) -> Result<impl Reply, Rejection> {
    let mut parts = form.into_stream();

    while let Some(Ok(p)) = parts.next().await {
        if p.name() != "file" {
          continue;
        }

        let content_type = p.content_type();

        match content_type {
            Some(file_type) => match file_type {
                "application/pdf" => (),
                "application/epub+zip" => (),
                "text/html" => (),
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
        
        let document_hash = generate_hash(&value);

        let file_path = &format!("./{uploads_dir}/{document_hash}");

        let file_path_exists = tokio::fs::try_exists(file_path).await.unwrap();

        if (!file_path_exists) {
          tokio::fs::write(file_path, value).await.map_err(|e| {
              eprint!("error writing file: {}", e);
              warp::reject::reject()
          })?;

          let txn = db.start_transaction().unwrap();
          let collection = txn.collection::<HyggDocument>(db_collection_name);

          collection.insert_one(HyggDocument {
            timestamp: chrono::Utc::now(), 

            name: name.clone(),
            document_hash: document_hash.to_string(),
            path: file_path.into(),
          }).unwrap();

          txn.commit().unwrap();

          println!("created file: {}", file_path);
        }
    }

    Ok("success")
}
