// This is the most shittiest code but it works lmao

fn has_flag(flags: Vec<String>, arguments: &Vec<String>) -> bool {
  return flags.iter().any(|flag| arguments.contains(flag));
}

fn get_flags(arguments: &Vec<String>) -> Vec<String> {
  let mut flags: Vec<String> = Vec::new();
  for argument in arguments {
    if argument.starts_with("-") {
      flags.push(argument.clone());
    }
  }
  return flags;
}

fn main() {
  let arguments: Vec<String> = std::env::args().collect();
  
  let help_flag: bool = has_flag(vec!["--help".to_string()], &arguments);
  let version_flag: bool = has_flag(vec!["--version".to_string()], &arguments);

  let flags = get_flags(&arguments);
  let valid_flags = vec!["--help".to_string(), "--version".to_string()];

  let expletive = arguments.get(1)
    .unwrap_or(&"n".to_string()).clone();

  for flag in flags {
    if !valid_flags.contains(&flag) {
      println!("no: invalid option -- '{}'", flag);
      println!("Try 'no --help' for more information.");

      std::process::exit(1);
    }
  }
  
  if help_flag {
    println!("Usage: no [STRING]...");
    println!("or:  no OPTION");
    println!("Repeatedly output a line with all specified STRING(s), or 'n'.\n");
    println!("  --help     display this help and exit");
    println!("  --version  output version information and exit\n");
    println!("GNU ballutils online help: <doesn't exist :troll:>");
    println!("Report any translation bugs to <nope>");
    println!("Full documentation <no documents to ever be made>");
    println!("or available locally via: info '(ballutils) no invocation'");

    std::process::exit(0);
  }

  if version_flag {
    println!("no (GNU ballutils) 1.0");
    println!("Copyright (C) 2024 Free Software Foundation, Inc.");
    println!("License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.");
    println!("This is free software: you are free to change and redistribute it."); 
    println!("There is NO WARRANTY, to the extent permitted by law.\n");
    println!("Written by metamethods.");

    std::process::exit(0);
  }

  loop {
    println!("{expletive}");
  }
}
