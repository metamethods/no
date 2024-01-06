use std::{ thread, time };

use clap::Parser;

/// Repeatedly output a line with all specified STRING(s), or 'n'.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  strings: Option<Vec<String>>,

  /// Wait x milliseconds between each line.
  #[arg(short, long)]
  interval: Option<u64>,

  /// Yes
  #[arg(short, long)]
  yes: bool,

  /// Count
  #[arg(short, long)]
  count: Option<u32>
}

fn main() {
  let args = Args::parse();
  let string = args.strings
    .unwrap_or(vec!["n".to_string()])
    .join(" ");

  let mut current_count: u32 = 0;

  loop {
    current_count += 1;

    if args.yes {
      println!("y");
    } else {
      println!("{string}");
    }

    if let Some(interval) = args.interval {
      thread::sleep(time::Duration::from_millis(interval));
    }

    if let Some(max_count) = args.count {
      if current_count >= max_count  {
        break;
      }
    }
  }
}