use clap::{Arg, ArgMatches, Command};

pub fn parse() -> ArgMatches {
    Command::new("fitx")
        .version(clap::crate_version!())
        .author("Éverton M. Vieira <everton.muvi@gmail.com>")
        .about("Fitx (File System Toolbox) is a library and a command program that features a toolbox with a series of file system functionalities.")
        .subcommand(
            Command::new("compare")
                .about("Compares two path, looking for differences.")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("PATH")
                        .takes_value(true)
                        .required(true)
                        .help("The path origin for comparison."),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
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
                ),
        )
        .get_matches()
}
