use clap::Parser;

/// Repeatedly output a line with all specified STRING(s), or 'n'.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  strings: Option<Vec<String>>,
}

fn main() {
  loop {
    println!(
      "{}", 
      Args::parse().strings
        .unwrap_or(vec!["n".to_string()])
        .join(" ")
    );
  }
}