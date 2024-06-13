use cli_justify;
use cli_pdf_to_text;

use std::env;
use std::io::{self, BufRead, Write};

use crossterm::{
  cursor::{Hide, MoveTo, Show},
  event::{self, Event, KeyCode, KeyEvent},
  execute,
  terminal::{self, Clear, ClearType},
};
use getopts;
use linefeed::{Interface, ReadResult};

fn handle_command(command: String) {
  if command == "q" {
    std::process::exit(0);
  }
  if command == ":q" {
    std::process::exit(0);
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut stdout = io::stdout();

  let args: Vec<String> = env::args().collect();
  let mut opts = getopts::Options::new();

  opts.optopt("c", "col", "set the column, defaults to 80", "NUMBER");
  opts.optflag("h", "help", "print this help menu");

  let matches = opts.parse(&args[1..])?;

  if matches.opt_present("h") {
    let brief = format!("Usage: {} FILE [options]", args[0]);
    print!("{}", opts.usage(&brief));
    return Ok(());
  }

  let col: usize = match matches.opt_str("c") {
    Some(x) => x.parse().unwrap_or(80),
    None => 80,
  };

  let file = std::env::args().nth(1).unwrap();
  let lines = cli_justify::justify(&cli_pdf_to_text::pdf_to_text(&file), col);

  let total_lines = lines.len();
  let mut offset = 0;

  let mut width = terminal::size()?.0 as usize;
  let mut height = terminal::size()?.1 as usize;

  if atty::is(atty::Stream::Stdout) {
    execute!(stdout, terminal::EnterAlternateScreen, Hide)?;
    terminal::enable_raw_mode()?;
  }

  loop {
    if atty::is(atty::Stream::Stdout) {
      execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;
    }

    let mut show_line_number = false;
    let mut center = true;

    let center_offset = (width / 2) - col / 2;

    let mut center_offset_string =
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

    if atty::is(atty::Stream::Stdout) {
      match event::read()? {
        Event::Key(key_event) => match key_event.code {
          KeyCode::Char('j') => {
            if offset + height < total_lines {
              offset += 1;
            }
          }
          KeyCode::Char('k') => {
            if offset > 0 {
              offset -= 1;
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

  if atty::is(atty::Stream::Stdout) {
    execute!(stdout, Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
  }
  Ok(())
}
