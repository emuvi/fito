use regex::Regex;
use rubx::RubxResult;

mod clip;

fn main() -> RubxResult<()> {
  let args = clip::parse();
  if let Some(matches) = args.subcommand_matches("show") {
    let path = matches.value_of("path").unwrap();
    fitx::show::info(path.into())?;
  }
  if let Some(matches) = args.subcommand_matches("find") {
    let from = matches.value_of("from").unwrap();
    let verbose = matches.is_present("verbose");
    let all_extensions = matches.is_present("all-extensions");
    let by_name = if matches.is_present("by-name") {
      let by_name = matches.value_of("by-name").unwrap();
      let regex = Regex::new(by_name)?;
      Some(regex)
    } else {
      None
    };
    fitx::find::start(
      from.into(),
      fitx::find::Setup {
        verbose,
        all_extensions,
        by_name,
      },
    )?;
    if verbose {
      println!("Finish to find.");
    }
  }
  if let Some(matches) = args.subcommand_matches("compare") {
    let side_a = matches.value_of("side-a").unwrap();
    let side_b = matches.value_of("side-b").unwrap();
    let only_diffs = matches.is_present("only-diffs");
    let check_size = matches.is_present("check-size");
    let check_created = matches.is_present("check-created");
    let check_modified = matches.is_present("check-modified");
    let check_accessed = matches.is_present("check-accessed");
    let check_all = !check_size && !check_created && !check_modified && !check_accessed;
    fitx::compare::start(
      side_a.into(),
      side_b.into(),
      fitx::compare::Setup {
        only_diffs,
        check_size,
        check_created,
        check_modified,
        check_accessed,
        check_all,
      },
    )?;
    println!("Finish to compare.");
  }
  Ok(())
}
