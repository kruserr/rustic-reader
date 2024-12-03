use bytes::BufMut;
// use cli_text_reader_online::progress::Progress;
use futures::{StreamExt, TryStreamExt};
use tracing_subscriber::fmt::format::FmtSpan;
use std::convert::Infallible;
use uuid::Uuid;
use warp::{http::StatusCode, multipart::FormData, Filter, Rejection, Reply};
use serde_derive::{Deserialize, Serialize};

pub fn mkdir(
  input: &str,
) -> Result<String, Box<dyn std::error::Error + Send>> {
  let path = std::path::Path::new(input);
  std::fs::create_dir_all(path)
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;
  Ok("".into())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Progress {
  pub session_id: String,
  pub document_hash: u64,
  pub offset: usize,
  pub total_lines: usize,
  pub percentage: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Opened {
  pub session_id: String,
  pub document_hash: u64,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_span_events(FmtSpan::CLOSE).init();

    let uploads_dir = "hygg-server-uploads";
    mkdir(uploads_dir);

    let upload_route = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(50_000_000))
        .and_then(move |x| upload(x, &uploads_dir))
    ;

    let download_route = warp::path("download")
      .and(warp::fs::dir(uploads_dir))
    ;

    let progress_route = warp::post()
        .and(warp::path("progress"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|progress: Progress| {
            println!("{:?}", progress);

            warp::reply::json(&progress)
        })
    ;

    let opened_route = warp::post()
        .and(warp::path("opened"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|opened: Opened| {
            println!("{:?}", opened);

            warp::reply::json(&opened)
        })
    ;

    let router = upload_route
      .or(download_route)
      .or(opened_route)
      .or(progress_route)
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
