use chrono::{DateTime, Utc};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher};
use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;
use std::fs;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event as CEvent, KeyCode, KeyEvent},
    execute,
    terminal::{self, Clear, ClearType},
    style::{SetAttribute, Attribute, SetForegroundColor, SetBackgroundColor, Color, ResetColor},
};

use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct Progress {
  document_hash: u64,
  offset: usize,
  total_lines: usize,
  percentage: f64,
}

#[derive(Serialize, Deserialize)]
enum Event {
  UpdateProgress {
    timestamp: DateTime<Utc>,
    document_hash: u64,
    offset: usize,
    total_lines: usize,
    percentage: f64,
  },
}

fn generate_hash<T: Hash>(t: &T) -> u64 {
  let mut s = DefaultHasher::new();
  t.hash(&mut s);
  s.finish()
}

fn get_progress_file_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
  let mut config_path =
    config_dir().ok_or("Unable to find config directory")?;
  config_path.push("hygg");
  std::fs::create_dir_all(&config_path)?;
  config_path.push(".progress.jsonl");
  Ok(config_path)
}

fn save_progress(
  document_hash: u64,
  offset: usize,
  total_lines: usize,
) -> Result<(), Box<dyn std::error::Error>> {
  let percentage = (offset as f64 / total_lines as f64) * 100.0;
  let event = Event::UpdateProgress {
    timestamp: Utc::now(),
    document_hash,
    offset,
    total_lines,
    percentage,
  };
  let serialized = serde_json::to_string(&event)?;
  let progress_file_path = get_progress_file_path()?;
  let mut file =
    OpenOptions::new().create(true).append(true).open(progress_file_path)?;
  file.write_all(serialized.as_bytes())?;
  file.write_all(b"\n")?;
  Ok(())
}

fn load_progress(
  document_hash: u64,
) -> Result<Progress, Box<dyn std::error::Error>> {
  let progress_file_path = get_progress_file_path()?;
  let file = OpenOptions::new().read(true).open(progress_file_path)?;
  let reader = io::BufReader::new(file);
  let mut latest_progress: Option<Progress> = None;

  for line in reader.lines() {
    let line = line?;
    let event: Event = serde_json::from_str(&line)?;
    #[allow(irrefutable_let_patterns)]
    if let Event::UpdateProgress {
      document_hash: hash,
      offset,
      total_lines,
      percentage,
      ..
    } = event
    {
      if hash == document_hash {
        latest_progress = Some(Progress {
          document_hash: hash,
          offset,
          total_lines,
          percentage,
        });
      }
    }
  }

  latest_progress
    .ok_or_else(|| "No progress found for the given document hash".into())
}

fn get_tutorial_text() -> Vec<String> {
  vec![
    "Welcome to the Text Reader!".to_string(),
    "".to_string(),
    "Basic Controls:".to_string(),
    "  j or ↓    : Move down one line".to_string(),
    "  k or ↑    : Move up one line".to_string(),
    "  PageDown  : Move down one page".to_string(),
    "  PageUp    : Move up one page".to_string(),
    "  :z        : Toggle line highlighter".to_string(),
    "  :q        : Quit".to_string(),
    "".to_string(),
    "Press any key to continue...".to_string(),
  ]
}

#[derive(Default)]
struct AppConfig {
    enable_tutorial: Option<bool>,
    enable_line_highlighter: Option<bool>,
}

fn get_config_env_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut config_path = config_dir().ok_or("Unable to find config directory")?;
    config_path.push("hygg");
    std::fs::create_dir_all(&config_path)?;
    config_path.push(".env");
    Ok(config_path)
}

fn ensure_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_env_path()?;
    if (!config_path.exists()) {
        fs::write(config_path, "ENABLE_TUTORIAL=true\nENABLE_LINE_HIGHLIGHTER=true\n")?;
    }
    Ok(())
}

fn load_config() -> AppConfig {
    let mut config = AppConfig::default();
    
    if let Ok(config_path) = get_config_env_path() {
        if let Ok(_) = ensure_config_file() {
            dotenvy::from_path(config_path).ok();
            if let Ok(val) = std::env::var("ENABLE_TUTORIAL") {
                config.enable_tutorial = Some(val.to_lowercase() == "true");
            }
            if let Ok(val) = std::env::var("ENABLE_LINE_HIGHLIGHTER") {
                config.enable_line_highlighter = Some(val.to_lowercase() == "true");
            }
        }
    }
    
    config
}

#[derive(PartialEq)]
enum EditorMode {
    Normal,
    Command,
}

struct EditorState {
    mode: EditorMode,
    command_buffer: String,
}

fn handle_command(command: &str, show_highlighter: &mut bool) -> bool {
    match command.trim() {
        "q" => true,  // return true to quit
        "z" => {
            *show_highlighter = !*show_highlighter;
            false
        }
        _ => false,
    }
}

pub fn run_cli_text_reader(
  lines: Vec<String>,
  col: usize,
) -> Result<(), Box<dyn std::error::Error>> {
  let mut stdout = io::stdout();
  
  // Load config and check if tutorial should be shown
  let config = load_config();
  let document_hash = generate_hash(&lines);
  let show_tutorial = match config.enable_tutorial {
      Some(false) => false,
      _ => lines.is_empty() || load_progress(document_hash).is_err(),
  };
  
  if show_tutorial {
    let tutorial_lines = get_tutorial_text();
    let mut stdout = io::stdout();
    
    if std::io::stdout().is_terminal() {
      execute!(stdout, terminal::EnterAlternateScreen, Hide)?;
      terminal::enable_raw_mode()?;
      
      let width = terminal::size()?.0 as usize;
      let height = terminal::size()?.1 as usize;
      let center_offset = if width > col { (width / 2) - col / 2 } else { 0 };
      
      execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;
      
      for (i, line) in tutorial_lines.iter().enumerate() {
        execute!(stdout, MoveTo(center_offset as u16, (height/2 - tutorial_lines.len()/2 + i) as u16))?;
        println!("{}", line);
      }
      
      stdout.flush()?;
      
      // Wait for any key press
      while let Ok(event) = event::read() {
        if let CEvent::Key(_) = event {
          break;
        }
      }
    }
  }

  // If the file is empty, exit after tutorial
  if lines.is_empty() {
    if std::io::stdout().is_terminal() {
      execute!(stdout, Show, terminal::LeaveAlternateScreen)?;
      terminal::disable_raw_mode()?;
    }
    return Ok(());
  }

  let total_lines = lines.len();
  let mut offset = match load_progress(document_hash) {
    Ok(progress) => {
      (progress.percentage / 100.0 * total_lines as f64).round() as usize
    }
    Err(_) => 0,
  };

  let mut width = terminal::size()?.0 as usize;
  let mut height = terminal::size()?.1 as usize;

  if std::io::stdout().is_terminal() {
    execute!(stdout, terminal::EnterAlternateScreen, Hide)?;
    terminal::enable_raw_mode()?;
  }

  let mut show_highlighter = config.enable_line_highlighter.unwrap_or(true);

  let mut editor_state = EditorState {
    mode: EditorMode::Normal,
    command_buffer: String::new(),
  };

  loop {
    if std::io::stdout().is_terminal() {
      execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;
    }

    let show_line_number = false;
    let center = true;
    let term_width = terminal::size()?.0 as u16;
    let center_offset = if width > col { (width / 2) - col / 2 } else { 0 };
    let center_offset_string = if center { " ".repeat(center_offset) } else { "".to_string() };

    for (i, line_orig) in lines.iter().skip(offset).take(height).enumerate() {
      let mut line = line_orig.clone();
      if show_line_number {
        line = format!("{i} {line}");
      }

      execute!(stdout, MoveTo(0, i as u16))?;

      // Draw highlight if enabled
      if show_highlighter && i == height / 2 {
        execute!(
            stdout,
            SetBackgroundColor(Color::Rgb { r: 40, g: 40, b: 40 })
        )?;
        print!("{}", " ".repeat(term_width as usize));
        execute!(stdout, MoveTo(0, i as u16))?;
      }

      println!("{}{}", center_offset_string, line);
      
      if show_highlighter && i == height / 2 {
        execute!(stdout, SetBackgroundColor(Color::Reset))?;
      }
    }

    // Add command line display at the bottom
    if editor_state.mode == EditorMode::Command {
        execute!(stdout, MoveTo(0, (height - 1) as u16))?;
        print!(":{}", editor_state.command_buffer);
    }

    stdout.flush()?;

    if std::io::stdout().is_terminal() {
      match event::read()? {
        CEvent::Key(key_event) => match editor_state.mode {
            EditorMode::Normal => match key_event.code {
                KeyCode::Char(':') => {
                    editor_state.mode = EditorMode::Command;
                    editor_state.command_buffer.clear();
                },
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
                _ => {}
            },
            EditorMode::Command => match key_event.code {
                KeyCode::Esc => {
                    editor_state.mode = EditorMode::Normal;
                    editor_state.command_buffer.clear();
                },
                KeyCode::Enter => {
                    if handle_command(&editor_state.command_buffer.trim_start_matches(':'), &mut show_highlighter) {
                        break;
                    }
                    editor_state.mode = EditorMode::Normal;
                    editor_state.command_buffer.clear();
                },
                KeyCode::Backspace => {
                    editor_state.command_buffer.pop();
                },
                KeyCode::Char(c) => {
                    editor_state.command_buffer.push(c);
                },
                _ => {}
            }
        },
        CEvent::Resize(_, _) => {
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
