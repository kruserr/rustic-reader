use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::thread::sleep;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;

pub fn create_server_http(port: u16) -> Result<(), Box<dyn Error>> {
  tracing_subscriber::fmt().with_span_events(FmtSpan::CLOSE).init();

  let rt = tokio::runtime::Runtime::new()?;
  rt.block_on(
    warp::serve(warp::fs::dir(".").with(warp::trace::request()))
      .run(([0, 0, 0, 0], port)),
  );

  Ok(())
}

pub fn main() {
  create_server_http(3030);
}
