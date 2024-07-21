use cli_epub_to_text;
use std;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let file_path = std::env::args().nth(1).unwrap();
  println!("{}", cli_epub_to_text::epub_to_text(&file_path)?);

  return Ok(());
}
