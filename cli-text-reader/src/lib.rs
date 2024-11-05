use std::io::{self, IsTerminal, Write};
use std::fs::OpenOptions;
use serde::{Serialize, Deserialize};

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

use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Progress {
  offset: usize,
  total_lines: usize,
  percentage: f64,
}

fn save_progress(offset: usize, total_lines: usize) -> Result<(), Box<dyn std::error::Error>> {
  let percentage = (offset as f64 / total_lines as f64) * 100.0;
  let progress = Progress { offset, total_lines, percentage };
  let serialized = serde_json::to_string(&progress)?;
  let mut file = OpenOptions::new().create(true).write(true).truncate(true).open("progress.json")?;
  file.write_all(serialized.as_bytes())?;
  Ok(())
}

fn load_progress() -> Result<Progress, Box<dyn std::error::Error>> {
  let mut file = OpenOptions::new().read(true).open("progress.json")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let progress: Progress = serde_json::from_str(&contents)?;
  Ok(progress)
}

pub fn run_cli_text_reader(
  lines: Vec<String>,
  col: usize,
) -> Result<(), Box<dyn std::error::Error>> {
  let mut stdout = io::stdout();

  let total_lines = lines.len();
  let mut offset = match load_progress() {
    Ok(progress) => (progress.percentage / 100.0 * total_lines as f64).round() as usize,
    Err(_) => 0,
  };

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
      if center { " ".repeat(center_offset) } else { "".to_string() };

    for (i, line_orig) in lines.iter().skip(offset).take(height).enumerate() {
      let mut line = line_orig.clone();

      execute!(stdout, MoveTo(0, i as u16))?;

      if show_line_number {
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

    save_progress(offset, total_lines)?;
  }

  if std::io::stdout().is_terminal() {
    execute!(stdout, Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
  }
  Ok(())
}