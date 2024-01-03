use std::{ thread, time };

use clap::Parser;

/// Repeatedly output a line with all specified STRING(s), or 'n'.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  strings: Option<Vec<String>>,

  /// Wait x milliseconds between each line.
  #[arg(short, long)]
  interval: Option<u64>
}

fn main() {
  let string = Args::parse().strings
    .unwrap_or(vec!["n".to_string()])
    .join(" ");

  let interval = Args::parse().interval;

  loop {
    println!("{string}");

    if let Some(interval) = interval {
      thread::sleep(time::Duration::from_millis(interval));
    }
  }
}