use clap::{ command, Arg, ArgAction };
use std::io;
use std::fs::{self, File};

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

pub fn display(file: &str, input: &Input) -> (usize, usize, usize, usize)  {
  if file == "-" {
    display_from_stdin(input)
  } else {
    display_from_file(file, input)
  }
}

fn display_from_stdin(input: &Input) -> (usize, usize, usize, usize) {
  let stdin = io::read_to_string(io::stdin()).unwrap();

  calculate_counts(input, stdin, None)
}

fn display_from_file(file: &str, input: &Input) -> (usize, usize, usize, usize) {
  if let Err(error) = File::open(file) {
    eprintln!("wc: {}: {}", file, error);
    return (0, 0, 0, 0)
  }

  let content = fs::read_to_string(file).unwrap();
  calculate_counts(input, content, Some(file))
}

fn calculate_counts(input: &Input, content: String, file: Option<&str>) -> (usize, usize, usize, usize) {
  let byte_len = content.len();
  let char_len = content.chars().count();
  let word_len = content.trim().split(' ').collect::<Vec<&str>>().len();
  let line_len = content.split('\n').collect::<Vec<&str>>().len() - 1;

  let mut result: Vec<usize> = Vec::new();

  if input.line_count { result.push(line_len); }
  if input.word_count { result.push(word_len); }
  if input.byte_count { result.push(byte_len); }
  if input.character_count { result.push(char_len); }

  if !input.line_count && !input.word_count && !input.byte_count && !input.character_count {
    match file {
      Some(file) => println!("\t{}\t{}\t{}\t{}", line_len, word_len, byte_len, file),
      None => println!("\t{}\t{}\t{}", line_len, word_len, byte_len)
    }
  } else {
    let output: Vec<String> = result.iter().map(|num| num.to_string()).collect();

    println!("\t{}", output.join("\t"));
  }

  (line_len, word_len, char_len, byte_len)
}
