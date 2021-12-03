use clap::{App, Arg, ArgMatches, SubCommand};

pub fn parse<'a>() -> ArgMatches<'a> {
    App::new("fito")
        .version(clap::crate_version!())
        .author("Éverton M. Vieira <everton.muvi@gmail.com>")
        .about("FileSystem ToolBox.")
        .subcommand(
            SubCommand::with_name("compare")
                .about("controls testing features")
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
                    Arg::with_name("only_diffs")
                        .long("only_diffs")
                        .takes_value(false)
                        .required(false)
                        .help("Prints only the differences."),
                )
                .arg(
                    Arg::with_name("check_size")
                        .long("check_size")
                        .takes_value(false)
                        .required(false)
                        .help("Checks for the size of the files."),
                )
                .arg(
                    Arg::with_name("check_created")
                        .long("check_created")
                        .takes_value(false)
                        .required(false)
                        .help("Checks for the created time of the files."),
                )
                .arg(
                    Arg::with_name("check_modified")
                        .long("check_modified")
                        .takes_value(false)
                        .required(false)
                        .help("Checks for the modified time of the files."),
                )
                .arg(
                    Arg::with_name("check_accessed")
                        .long("check_accessed")
                        .takes_value(false)
                        .required(false)
                        .help("Checks for the accessed time of the files."),
                ),
        )
        .get_matches()
}
