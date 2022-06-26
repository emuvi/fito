use clap::{Arg, ArgMatches, Command};

pub fn parse() -> ArgMatches {
  Command::new("fitx")
    .version(clap::crate_version!())
    .author("Éverton M. Vieira <emuvi@outlook.com.br>")
    .about("Fitx (File System Toolbox) is a library and a command program that features a toolbox with a series of file system functionalities.")
    .subcommand(cmd_show())
    .subcommand(cmd_find())
    .subcommand(cmd_compare())
    .get_matches()
}

fn cmd_show<'a>() -> Command<'a> {
  Command::new("show")
    .about("Shows the information about a path.")
    .arg(
      Arg::new("path")
        .short('p')
        .long("path")
        .value_name("PATH")
        .takes_value(true)
        .required(true)
        .help("Of what path the information should be show."),
    )
}

fn cmd_find<'a>() -> Command<'a> {
  Command::new("find")
    .about("Finds files and folders according with setup.")
    .arg(
      Arg::new("verbose")
        .short('v')
        .long("verbose")
        .takes_value(false)
        .required(false)
        .help("Prints the verbose messages."),
    )
    .arg(
      Arg::new("from")
        .short('f')
        .long("from")
        .value_name("PATH")
        .takes_value(true)
        .required(true)
        .help("The root path for find starts to look."),
    )
    .arg(
      Arg::new("all-extensions")
        .short('e')
        .long("all-extensions")
        .takes_value(false)
        .required(false)
        .help("Get a list of all extensions on the folder tree."),
    )
    .arg(
      Arg::new("by-name")
        .short('n')
        .long("by-name")
        .takes_value(true)
        .value_name("REGEX")
        .required(false)
        .help("Get a list of all files which names satisfies a REGEX expression."),
    )
}

fn cmd_compare<'a>() -> Command<'a> {
  Command::new("compare")
    .about("Compares two path, looking for differences.")
    .arg(
      Arg::new("side-a")
        .short('a')
        .long("side-a")
        .value_name("PATH")
        .takes_value(true)
        .required(true)
        .help("The path origin for comparison."),
    )
    .arg(
      Arg::new("side-b")
        .short('b')
        .long("side-b")
        .value_name("PATH")
        .takes_value(true)
        .required(true)
        .help("The path destiny for comparison."),
    )
    .arg(
      Arg::new("only-diffs")
        .long("only-diffs")
        .takes_value(false)
        .required(false)
        .help("Prints only the differences."),
    )
    .arg(
      Arg::new("check-size")
        .long("check-size")
        .takes_value(false)
        .required(false)
        .help("Checks for the size of the files."),
    )
    .arg(
      Arg::new("check-created")
        .long("check-created")
        .takes_value(false)
        .required(false)
        .help("Checks for the created time of the files."),
    )
    .arg(
      Arg::new("check-modified")
        .long("check-modified")
        .takes_value(false)
        .required(false)
        .help("Checks for the modified time of the files."),
    )
    .arg(
      Arg::new("check-accessed")
        .long("check-accessed")
        .takes_value(false)
        .required(false)
        .help("Checks for the accessed time of the files."),
    )
}
