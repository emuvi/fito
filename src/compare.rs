use rubx::RubxResult;

use std::path::PathBuf;

pub struct Compare {
  pub only_diffs: bool,
  pub check_size: bool,
  pub check_created: bool,
  pub check_modified: bool,
  pub check_accessed: bool,
  pub check_all: bool,
}

pub fn compare(input: PathBuf, output: PathBuf, options: Compare) -> RubxResult<()> {
  if !input.exists() {
    println!("Input not exists.");
    return Ok(());
  }
  if !output.exists() {
    println!("Output not exists.");
    return Ok(());
  }
  if input.is_dir() {
    compare_dirs(input, output, &options)
  } else {
    compare_file(input, output, &options)
  }
}

fn compare_dirs(input: PathBuf, output: PathBuf, options: &Compare) -> RubxResult<()> {
  if !input.exists() {
    println!(
      "Comparing dirs: '{}' and '{}': Input not exists.",
      input.display(),
      output.display()
    );
    return Ok(());
  }
  if !input.is_dir() {
    println!(
      "Comparing dirs: '{}' and '{}': Input is not a directory.",
      input.display(),
      output.display()
    );
    return Ok(());
  }
  if !output.exists() {
    println!(
      "Comparing dirs: '{}' and '{}': Output not exists.",
      input.display(),
      output.display()
    );
    return Ok(());
  }
  if !output.is_dir() {
    println!(
      "Comparing dirs: '{}' and '{}': Output is not a directory.",
      input.display(),
      output.display()
    );
    return Ok(());
  }
  for origin in std::fs::read_dir(input)? {
    let origin = origin?;
    let file_type = origin.file_type()?;
    let destiny = output.join(origin.file_name());
    if file_type.is_dir() {
      compare_dirs(origin.path(), destiny, options)?;
    } else {
      compare_file(origin.path(), destiny, options)?;
    }
  }
  Ok(())
}

fn compare_file(input: PathBuf, output: PathBuf, options: &Compare) -> RubxResult<()> {
  if !input.exists() {
    println!(
      "Comparing file: '{}' with '{}': Input not exists.",
      input.display(),
      output.display()
    );
    return Ok(());
  }
  if !input.is_file() {
    println!(
      "Comparing file: '{}' with '{}': Input is not a file.",
      input.display(),
      output.display()
    );
    return Ok(());
  }
  if !output.exists() {
    println!(
      "Comparing file: '{}' with '{}': Output not exists.",
      input.display(),
      output.display()
    );
    return Ok(());
  }
  if !output.is_file() {
    println!(
      "Comparing file: '{}' with '{}': Output is not a file.",
      input.display(),
      output.display()
    );
    println!("");
    return Ok(());
  }
  let input_meta = input.metadata()?;
  let output_meta = output.metadata()?;
  let mut diff_size = false;
  let mut diff_created: Option<bool> = None;
  let mut diff_modified: Option<bool> = None;
  let mut diff_accessed: Option<bool> = None;
  if options.check_all || options.check_size {
    let input_size = input_meta.len();
    let output_size = output_meta.len();
    if input_size != output_size {
      diff_size = true;
    }
  }
  if options.check_all || options.check_created {
    if let Ok(input_time) = input_meta.created() {
      if let Ok(output_time) = output_meta.created() {
        if input_time != output_time {
          diff_created = Some(true);
        } else {
          diff_accessed = Some(false);
        }
      }
    }
  }
  if options.check_all || options.check_modified {
    if let Ok(input_time) = input_meta.modified() {
      if let Ok(output_time) = output_meta.modified() {
        if input_time != output_time {
          diff_modified = Some(true);
        } else {
          diff_accessed = Some(false);
        }
      }
    }
  }
  if options.check_all || options.check_accessed {
    if let Ok(input_time) = input_meta.accessed() {
      if let Ok(output_time) = output_meta.accessed() {
        if input_time != output_time {
          diff_accessed = Some(true);
        } else {
          diff_accessed = Some(false);
        }
      }
    }
  }
  let mut has_diffs = false;
  let mut result = String::new();
  result.push_str(&format!(
    "Comparing file: '{}' with '{}': They have",
    input.display(),
    output.display()
  ));
  if diff_size {
    result.push_str(&format!(" diff size"));
    has_diffs = true;
  }
  if let Some(true) = diff_created {
    result.push_str(&format!(" diff created"));
    has_diffs = true;
  }
  if let Some(true) = diff_modified {
    result.push_str(&format!(" diff modified"));
    has_diffs = true;
  }
  if let Some(true) = diff_accessed {
    result.push_str(&format!(" diff accessed"));
    has_diffs = true;
  }
  if !has_diffs {
    result.push_str(" all the same.");
  } else {
    result.push_str(".");
  }
  let should_print = !options.only_diffs || has_diffs;
  if should_print {
    println!("{}", result);
  }
  Ok(())
}
