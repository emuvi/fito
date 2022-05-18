use rubx::RubxResult;

mod clip;

fn main() -> RubxResult<()> {
  let args = clip::parse();
  if let Some(matches) = args.subcommand_matches("compare") {
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let only_diffs = matches.is_present("only-diffs");
    let check_size = matches.is_present("check-size");
    let check_created = matches.is_present("check-created");
    let check_modified = matches.is_present("check-modified");
    let check_accessed = matches.is_present("check-accessed");
    let check_all = !check_size && !check_created && !check_modified && !check_accessed;
    fitx::compare::compare(
      input.into(),
      output.into(),
      fitx::compare::Compare {
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
