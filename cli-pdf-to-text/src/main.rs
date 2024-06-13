use cli_pdf_to_text;
use std;

fn main() {
  let file = std::env::args().nth(1).unwrap();
  println!("{}", cli_pdf_to_text::pdf_to_text(&file));
}
