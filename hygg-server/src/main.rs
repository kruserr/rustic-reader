use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::thread::sleep;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;

pub fn mkdir(
  input: &str,
) -> Result<String, Box<dyn std::error::Error + Send>> {
  let path = std::path::Path::new(input);
  std::fs::create_dir_all(path)
    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;
  Ok("".into())
}

pub fn create_server_http(port: u16) -> Result<(), Box<dyn Error>> {
  tracing_subscriber::fmt().with_span_events(FmtSpan::CLOSE).init();

  let uploads_dir = "hygg-server-uploads";
  mkdir(uploads_dir);

  let rt = tokio::runtime::Runtime::new()?;
  rt.block_on(
    warp::serve(warp::fs::dir(uploads_dir).with(warp::trace::request()))
      .run(([0, 0, 0, 0], port)),
  );

  Ok(())
}

pub fn main() {
  create_server_http(3030);
}
