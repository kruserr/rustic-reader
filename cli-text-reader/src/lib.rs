use std::io::{self, IsTerminal, Write};
use std::fs::OpenOptions;
use serde::{Serialize, Deserialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use dirs::config_dir;

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
  document_hash: u64,
  offset: usize,
  total_lines: usize,
  percentage: f64,
}

fn generate_hash<T: Hash>(t: &T) -> u64 {
  let mut s = DefaultHasher::new();
  t.hash(&mut s);
  s.finish()
}

fn get_progress_file_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
  let mut config_path = config_dir().ok_or("Unable to find config directory")?;
  config_path.push("rustic-reader");
  std::fs::create_dir_all(&config_path)?;
  config_path.push(".progress.json");
  Ok(config_path)
}

fn save_progress(document_hash: u64, offset: usize, total_lines: usize) -> Result<(), Box<dyn std::error::Error>> {
  let percentage = (offset as f64 / total_lines as f64) * 100.0;
  let progress = Progress { document_hash, offset, total_lines, percentage };
  let serialized = serde_json::to_string(&progress)?;
  let progress_file_path = get_progress_file_path()?;
  let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(progress_file_path)?;
  file.write_all(serialized.as_bytes())?;
  Ok(())
}

fn load_progress(document_hash: u64) -> Result<Progress, Box<dyn std::error::Error>> {
  let progress_file_path = get_progress_file_path()?;
  let mut file = OpenOptions::new().read(true).open(progress_file_path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let progress: Progress = serde_json::from_str(&contents)?;
  if progress.document_hash == document_hash {
    Ok(progress)
  } else {
    Err("Document hash does not match".into())
  }
}

pub fn run_cli_text_reader(
  lines: Vec<String>,
  col: usize,
) -> Result<(), Box<dyn std::error::Error>> {
  let mut stdout = io::stdout();

  let document_hash = generate_hash(&lines);
  let total_lines = lines.len();
  let mut offset = match load_progress(document_hash) {
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

    save_progress(document_hash, offset, total_lines)?;
  }

  if std::io::stdout().is_terminal() {
    execute!(stdout, Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
  }
  Ok(())
}