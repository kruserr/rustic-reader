mod config;
mod progress;
mod tutorial;
mod editor;

use editor::Editor;

pub fn run_cli_text_reader(
    lines: Vec<String>,
    col: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut editor = Editor::new(lines, col);
    editor.run()
}
