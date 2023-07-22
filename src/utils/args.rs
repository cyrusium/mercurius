use clap::{value_parser, Arg, ArgAction, Command};

pub fn get_args() -> () {
  let before_help = format!(
    "{} {}\nLicense: {}\nSource: {}\nAuthors: {}",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION"),
    env!("CARGO_PKG_LICENSE"),
    env!("CARGO_PKG_HOMEPAGE"),
    env!("CARGO_PKG_AUTHORS").split(':').collect::<Vec<&str>>().join(", ")
  );

  let matches = Command::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .before_help(before_help)
    .name(env!("CARGO_PKG_NAME"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .arg(
      Arg::new("trace")
        .long("trace")
        .num_args(0)
        .default_value("false")
        .value_parser(value_parser!(bool))
        .action(ArgAction::SetTrue)
        .conflicts_with_all(&["debug", "error", "warn", "info", "off"])
        .help("Sets the logging level to trace"),
    )
    .arg(
      Arg::new("debug")
        .long("debug")
        .num_args(0)
        .action(ArgAction::SetTrue)
        .conflicts_with_all(&["trace", "error", "warn", "info", "off"])
        .help("Sets the logging level to debug"),
    )
    .arg(
      Arg::new("error")
        .long("error")
        .num_args(0)
        .action(ArgAction::SetTrue)
        .conflicts_with_all(&["trace", "debug", "warn", "info", "off"])
        .help("Sets the logging level to error"),
    )
    .arg(
      Arg::new("warn")
        .long("warn")
        .num_args(0)
        .action(ArgAction::SetTrue)
        .conflicts_with_all(&["trace", "debug", "error", "info", "off"])
        .help("Sets the logging level to warn"),
    )
    .arg(
      Arg::new("info")
        .long("info")
        .num_args(0)
        .action(ArgAction::SetTrue)
        .conflicts_with_all(&["trace", "debug", "error", "warn", "off"])
        .help("Sets the logging level to info (default)"),
    )
    .arg(
      Arg::new("off")
        .long("off")
        .num_args(0)
        .action(ArgAction::SetTrue)
        .conflicts_with_all(&["trace", "debug", "error", "warn", "info"])
        .help("Sets the logging level to off"),
    )
    .arg(
      Arg::new("trace-file")
        .long("trace-file")
        .num_args(0)
        .action(ArgAction::SetTrue)
        .conflicts_with("disable-log")
        .help("Sets the logging level to trace for the log file"),
    )
    .arg(
      Arg::new("disable-log")
        .long("disable-log")
        .num_args(0)
        .action(ArgAction::SetTrue)
        .conflicts_with("trace-file")
        .help("Disables the log file"),
    )
    .get_matches();

  if matches.get_flag("trace") {
    std::env::set_var("LOGGING_LEVEL", "TRACE");
  } else if matches.get_flag("debug") {
    std::env::set_var("LOGGING_LEVEL", "DEBUG");
  } else if matches.get_flag("warn") {
    std::env::set_var("LOGGING_LEVEL", "WARN");
  } else if matches.get_flag("error") {
    std::env::set_var("LOGGING_LEVEL", "ERROR");
  } else if matches.get_flag("off") {
    std::env::set_var("LOGGING_LEVEL", "OFF");
  }

  if matches.get_flag("trace-file") {
    std::env::set_var("FILE_LOGGING_LEVEL", "TRACE");
  } else if matches.get_flag("disable-log") {
    std::env::set_var("FILE_LOGGING_LEVEL", "OFF");
  }
}
