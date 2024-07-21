use cli_justify;
use cli_pdf_to_text;
use cli_text_reader;

use std::env;

use getopts;

pub fn print_help_menu(args: Vec<String>, opts: getopts::Options) {
  let brief = format!("Usage: {} FILE [options]", args[0]);
  print!("{}", opts.usage(&brief));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<String> = env::args().collect();
  let mut opts = getopts::Options::new();

  opts.optopt("c", "col", "set the column, defaults to 110", "NUMBER");
  opts.optflag("h", "help", "print this help menu");

  let matches = opts.parse(&args[1..])?;

  if (matches.opt_present("h") || args.len() < 2) {
    print_help_menu(args, opts);
    return Ok(());
  }

  let col: usize = match matches.opt_str("c") {
    Some(x) => x.parse().unwrap_or(110),
    None => 110,
  };

  let file = std::env::args().nth(1).unwrap();

  let content = cli_epub_to_text::epub_to_text(&file)
    .or(cli_pdf_to_text::pdf_to_text(&file))?;

  let lines = cli_justify::justify(&content, col);

  cli_text_reader::run_cli_text_reader(lines, col)?;

  Ok(())
}
