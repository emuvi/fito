use clap::{App, Arg, ArgMatches, SubCommand};

pub fn parse<'a>() -> ArgMatches<'a> {
    App::new("fito")
        .version(clap::crate_version!())
        .author("Éverton M. Vieira <everton.muvi@gmail.com>")
        .about("Fitx (File System Toolbox) is a library and a command program that features a toolbox with a series of file system functionalities.")
        .subcommand(
            SubCommand::with_name("compare")
                .about("Compares two path, looking for differences.")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .value_name("PATH")
                        .takes_value(true)
                        .required(true)
                        .help("The path origin for comparison."),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("PATH")
                        .takes_value(true)
                        .required(true)
                        .help("The path destiny for comparison."),
                )
                .arg(
                    Arg::with_name("only-diffs")
                        .long("only-diffs")
                        .takes_value(false)
                        .required(false)
                        .help("Prints only the differences."),
                )
                .arg(
                    Arg::with_name("check-size")
                        .long("check-size")
                        .takes_value(false)
                        .required(false)
                        .help("Checks for the size of the files."),
                )
                .arg(
                    Arg::with_name("check-created")
                        .long("check-created")
                        .takes_value(false)
                        .required(false)
                        .help("Checks for the created time of the files."),
                )
                .arg(
                    Arg::with_name("check-modified")
                        .long("check-modified")
                        .takes_value(false)
                        .required(false)
                        .help("Checks for the modified time of the files."),
                )
                .arg(
                    Arg::with_name("check-accessed")
                        .long("check-accessed")
                        .takes_value(false)
                        .required(false)
                        .help("Checks for the accessed time of the files."),
                ),
        )
        .get_matches()
}
