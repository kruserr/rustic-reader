mod config;
mod editor;
pub mod progress;
mod tutorial;

use editor::Editor;

pub fn run_cli_text_reader(
  lines: Vec<String>,
  col: usize,
  document_hash: u64,
) -> Result<(), Box<dyn std::error::Error>> {
  let mut editor = Editor::new(lines, col, document_hash);
  editor.run()
}
