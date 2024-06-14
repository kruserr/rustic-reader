use lopdf;
use pdf_extract;
use std::{
  env,
  io::{BufWriter, Cursor},
};

pub fn pdf_to_text(
  pdf_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
  let path = std::path::Path::new(pdf_path);

  let mut output_buf = Vec::new();
  {
    let mut output_file = BufWriter::new(Cursor::new(&mut output_buf));

    let doc = lopdf::Document::load(path)?;

    pdf_extract::print_metadata(&doc);

    let mut output = Box::new(pdf_extract::PlainTextOutput::new(
      &mut output_file as &mut dyn std::io::Write,
    ));

    pdf_extract::output_doc(&doc, output.as_mut())?;
  }

  let res = std::str::from_utf8(&output_buf)
    .expect("Could not convert to String")
    .to_owned();

  return Ok(res);
}
