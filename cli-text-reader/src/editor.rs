use std::io::{self, IsTerminal, Write};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{self, Clear, ClearType},
    style::{SetBackgroundColor, Color},
};

use crate::config::load_config;
use crate::progress::{generate_hash, load_progress, save_progress};
use crate::tutorial::get_tutorial_text;

#[derive(PartialEq)]
pub enum EditorMode {
    Normal,
    Command,
}

pub struct EditorState {
    pub mode: EditorMode,
    pub command_buffer: String,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            mode: EditorMode::Normal,
            command_buffer: String::new(),
        }
    }
}

pub struct Editor {
    lines: Vec<String>,
    col: usize,
    offset: usize,
    width: usize,
    height: usize,
    show_highlighter: bool,
    editor_state: EditorState,
    document_hash: u64,
    total_lines: usize,
}

impl Editor {
    pub fn new(lines: Vec<String>, col: usize) -> Self {
        let document_hash = generate_hash(&lines);
        let total_lines = lines.len();
        let (width, height) = terminal::size()
            .map(|(w, h)| (w as usize, h as usize))
            .unwrap_or((80, 24));

        Self {
            lines,
            col,
            offset: 0,
            width,
            height,
            show_highlighter: true,
            editor_state: EditorState::new(),
            document_hash,
            total_lines,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = io::stdout();
        let config = load_config();
        
        self.show_highlighter = config.enable_line_highlighter.unwrap_or(true);
        
        let show_tutorial = match config.enable_tutorial {
            Some(false) => false,
            _ => self.lines.is_empty() || load_progress(self.document_hash).is_err(),
        };

        if show_tutorial {
            self.show_tutorial(&mut stdout)?;
        }

        // If the file is empty, exit after tutorial
        if self.lines.is_empty() {
            self.cleanup(&mut stdout)?;
            return Ok(());
        }

        self.offset = match load_progress(self.document_hash) {
            Ok(progress) => {
                (progress.percentage / 100.0 * self.total_lines as f64).round() as usize
            }
            Err(_) => 0,
        };

        if std::io::stdout().is_terminal() {
            execute!(stdout, terminal::EnterAlternateScreen, Hide)?;
            terminal::enable_raw_mode()?;
        }

        self.main_loop(&mut stdout)?;

        self.cleanup(&mut stdout)?;
        Ok(())
    }

    fn show_tutorial(&self, stdout: &mut io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
        let tutorial_lines = get_tutorial_text();
        
        if std::io::stdout().is_terminal() {
            execute!(stdout, terminal::EnterAlternateScreen, Hide)?;
            terminal::enable_raw_mode()?;
            
            let center_offset = if self.width > self.col { (self.width / 2) - self.col / 2 } else { 0 };
            
            execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;
            
            for (i, line) in tutorial_lines.iter().enumerate() {
                execute!(stdout, MoveTo(center_offset as u16, (self.height/2 - tutorial_lines.len()/2 + i) as u16))?;
                println!("{}", line);
            }
            
            stdout.flush()?;
            
            while let Ok(event) = event::read() {
                if let CEvent::Key(_) = event {
                    break;
                }
            }
        }
        
        Ok(())
    }

    fn cleanup(&self, stdout: &mut io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
        if std::io::stdout().is_terminal() {
            execute!(stdout, Show, terminal::LeaveAlternateScreen)?;
            terminal::disable_raw_mode()?;
        }
        Ok(())
    }

    fn main_loop(&mut self, stdout: &mut io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            if std::io::stdout().is_terminal() {
                execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;
            }

            let center = true;
            let term_width = terminal::size()?.0 as u16;
            let center_offset = if self.width > self.col { (self.width / 2) - self.col / 2 } else { 0 };
            let center_offset_string = if center { " ".repeat(center_offset) } else { "".to_string() };

            for (i, line_orig) in self.lines.iter().skip(self.offset).take(self.height).enumerate() {
                let line = line_orig.clone();
                execute!(stdout, MoveTo(0, i as u16))?;

                if self.show_highlighter && i == self.height / 2 {
                    execute!(
                        stdout,
                        SetBackgroundColor(Color::Rgb { r: 40, g: 40, b: 40 })
                    )?;
                    print!("{}", " ".repeat(term_width as usize));
                    execute!(stdout, MoveTo(0, i as u16))?;
                }

                println!("{}{}", center_offset_string, line);
                
                if self.show_highlighter && i == self.height / 2 {
                    execute!(stdout, SetBackgroundColor(Color::Reset))?;
                }
            }

            if self.editor_state.mode == EditorMode::Command {
                execute!(stdout, MoveTo(0, (self.height - 1) as u16))?;
                print!(":{}", self.editor_state.command_buffer);
            }

            stdout.flush()?;

            if std::io::stdout().is_terminal() {
                match event::read()? {
                    CEvent::Key(key_event) => match self.editor_state.mode {
                        EditorMode::Normal => match key_event.code {
                            KeyCode::Char(':') => {
                                self.editor_state.mode = EditorMode::Command;
                                self.editor_state.command_buffer.clear();
                            },
                            KeyCode::Char('j') | KeyCode::Down => {
                                if self.offset + self.height < self.total_lines {
                                    self.offset += 1;
                                }
                            }
                            KeyCode::Char('k') | KeyCode::Up => {
                                if self.offset > 0 {
                                    self.offset -= 1;
                                }
                            }
                            KeyCode::PageDown => {
                                if self.offset + self.height < self.total_lines {
                                    self.offset += self.height - 3;
                                }
                            }
                            KeyCode::PageUp => {
                                if self.offset as i32 - self.height as i32 > 0 {
                                    self.offset -= self.height - 3;
                                } else {
                                    self.offset = 0;
                                }
                            }
                            _ => {}
                        },
                        EditorMode::Command => match key_event.code {
                            KeyCode::Esc => {
                                self.editor_state.mode = EditorMode::Normal;
                                self.editor_state.command_buffer.clear();
                            },
                            KeyCode::Enter => {
                                if handle_command(&self.editor_state.command_buffer, &mut self.show_highlighter) {
                                    return Ok(());
                                }
                                self.editor_state.mode = EditorMode::Normal;
                                self.editor_state.command_buffer.clear();
                            },
                            KeyCode::Backspace => {
                                self.editor_state.command_buffer.pop();
                            },
                            KeyCode::Char(c) => {
                                self.editor_state.command_buffer.push(c);
                            },
                            _ => {}
                        }
                    },
                    CEvent::Resize(w, h) => {
                        self.width = w as usize;
                        self.height = h as usize;
                    }
                    _ => {}
                }
            } else {
                break;
            }

            save_progress(self.document_hash, self.offset, self.total_lines)?;
        }

        Ok(())
    }
}

pub fn handle_command(command: &str, show_highlighter: &mut bool) -> bool {
    match command.trim() {
        "q" => true,
        "z" => {
            *show_highlighter = !*show_highlighter;
            false
        }
        _ => false,
    }
}