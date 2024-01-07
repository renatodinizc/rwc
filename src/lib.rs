use clap::{ command, Arg, ArgAction };
use std::io::{self, BufRead, BufReader};
use std::fs::File;

#[derive(Debug)]
pub struct Input {
  pub files: Vec<String>,
  pub byte_count: bool,
  pub character_count: bool,
  pub word_count: bool,
  pub line_count: bool,
}

pub fn get_args() -> Input {
  let matches = command!()
    .arg(
        Arg::new("byte_count")
        .help("print the byte count")
        .short('c').long("bytes")
        .action(ArgAction::SetTrue)
    )
    .arg(
        Arg::new("character_count")
        .help("print the character count")
        .short('m').long("chars")
        .action(ArgAction::SetTrue)
    )
    .arg(
      Arg::new("word_count")
      .help("print the word count")
      .short('w').long("words")
      .action(ArgAction::SetTrue)
  )
    .arg(
      Arg::new("line_count")
      .help("print the newline count")
      .short('l').long("lines")
      .action(ArgAction::SetTrue)
  )
    .arg(Arg::new("files").action(ArgAction::Append).default_value("-"))
    .get_matches();

    Input {
      files: matches.get_many::<String>("files")
        .unwrap()
        .map(|v| v.to_string())
        .collect::<Vec<String>>(),
      byte_count: *matches.get_one::<bool>("byte_count").unwrap(),
      character_count: *matches.get_one::<bool>("character_count").unwrap(),
      word_count: *matches.get_one::<bool>("word_count").unwrap(),
      line_count: *matches.get_one::<bool>("line_count").unwrap(),
    }
}

pub fn display(file: &str, input: &Input) {
  if file == "-" {
    display_from_stdin(input);
  } else {
    display_from_file(file, input);
  }
}

fn display_from_stdin(input: &Input) {
  // let stdin = io::stdin();
  // for line in stdin.lines() {
  //     match line {
  //         Ok(content) => format_line_to_display(input),
  //         Err(error) => eprintln!("{error}"),
  //     }
  // }
}

fn display_from_file(file: &str, input: &Input) {
  if let Err(error) = File::open(file) { return eprintln!("rcat: {}: {}", file, error) }

  // let content = File::open(file).unwrap();
  // let buffer = BufReader::new(content);

  // for line in buffer.lines() {
  //     match line {
  //         Ok(sentence) => format_line_to_display(input),
  //         Err(error) => eprintln!("{error}"),
  //     }
  // }
}

fn format_line_to_display(input: &Input) {

}