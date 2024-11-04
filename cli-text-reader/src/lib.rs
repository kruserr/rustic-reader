use std::io::{self, IsTerminal, Write};

use crossterm::{
  cursor::{Hide, MoveTo, Show},
  event::{self, Event, KeyCode, KeyEvent},
  execute,
  terminal::{self, Clear, ClearType},
};

#[allow(dead_code)]
fn handle_command(command: String) {
  if command == "q" {
    std::process::exit(0);
  }
  if command == ":q" {
    std::process::exit(0);
  }
}

pub fn run_cli_text_reader(
  lines: Vec<String>,
  col: usize,
) -> Result<(), Box<dyn std::error::Error>> {
  // if (lines.len() < 2) {
  // // TODO: Print tutorial here
  // }

  let mut stdout = io::stdout();

  let total_lines = lines.len();
  let mut offset = 0;

  let mut width = terminal::size()?.0 as usize;
  let mut height = terminal::size()?.1 as usize;

  if std::io::stdout().is_terminal() {
    execute!(stdout, terminal::EnterAlternateScreen, Hide)?;
    terminal::enable_raw_mode()?;
  }

  loop {
    if std::io::stdout().is_terminal() {
      execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;
    }

    let show_line_number = false;
    let center = true;

    let center_offset = if width > col { (width / 2) - col / 2 } else { 0 };

    let center_offset_string =
      if (center) { " ".repeat(center_offset) } else { "".to_string() };

    for (i, line_orig) in lines.iter().skip(offset).take(height).enumerate() {
      let mut line = line_orig.clone();

      execute!(stdout, MoveTo(0, i as u16))?;

      if (show_line_number) {
        line = format!("{i} {line}");
      }

      println!("{center_offset_string}{line}");
    }

    stdout.flush()?;

    if std::io::stdout().is_terminal() {
      match event::read()? {
        Event::Key(key_event) => match key_event.code {
          KeyCode::Char('j') | KeyCode::Down => {
            if offset + height < total_lines {
              offset += 1;
            }
          }
          KeyCode::Char('k') | KeyCode::Up => {
            if offset > 0 {
              offset -= 1;
            }
          }
          KeyCode::PageDown => {
            if offset + height < total_lines {
              offset += height - 3;
            }
          }
          KeyCode::PageUp => {
            if offset as i32 - height as i32 > 0 {
              offset -= height - 3;
            } else {
              offset = 0;
            }
          }
          KeyCode::Char('q') => break,
          _ => {}
        },
        Event::Resize(_, _) => {
          width = terminal::size()?.0 as usize;
          height = terminal::size()?.1 as usize;
        }
        _ => {}
      }
    } else {
      break;
    }
  }

  if std::io::stdout().is_terminal() {
    execute!(stdout, Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
  }
  Ok(())
}
