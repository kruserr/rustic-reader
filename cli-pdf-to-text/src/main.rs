use cli_pdf_to_text;
use std;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let file = std::env::args().nth(1).unwrap();
  println!("{}", cli_pdf_to_text::pdf_to_text(&file)?);

  return Ok(());
}
