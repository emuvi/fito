use rubx::RubxResult;

use std::path::PathBuf;

pub struct Setup {
  pub only_diffs: bool,
  pub check_size: bool,
  pub check_created: bool,
  pub check_modified: bool,
  pub check_accessed: bool,
  pub check_all: bool,
}

pub fn start(side_a: PathBuf, side_b: PathBuf, setup: Setup) -> RubxResult<()> {
  if !side_a.exists() {
    println!("side_a not exists.");
    return Ok(());
  }
  if !side_b.exists() {
    println!("side_b not exists.");
    return Ok(());
  }
  if side_a.is_dir() {
    compare_dirs(side_a, side_b, &setup)
  } else {
    compare_file(side_a, side_b, &setup)
  }
}

fn compare_dirs(side_a: PathBuf, side_b: PathBuf, setup: &Setup) -> RubxResult<()> {
  if !side_a.exists() {
    println!(
      "Comparing dirs: '{}' and '{}': side_a not exists.",
      side_a.display(),
      side_b.display()
    );
    return Ok(());
  }
  if !side_a.is_dir() {
    println!(
      "Comparing dirs: '{}' and '{}': side_a is not a directory.",
      side_a.display(),
      side_b.display()
    );
    return Ok(());
  }
  if !side_b.exists() {
    println!(
      "Comparing dirs: '{}' and '{}': side_b not exists.",
      side_a.display(),
      side_b.display()
    );
    return Ok(());
  }
  if !side_b.is_dir() {
    println!(
      "Comparing dirs: '{}' and '{}': side_b is not a directory.",
      side_a.display(),
      side_b.display()
    );
    return Ok(());
  }
  for origin in std::fs::read_dir(side_a)? {
    let origin = origin?;
    let file_type = origin.file_type()?;
    let destiny = side_b.join(origin.file_name());
    if file_type.is_dir() {
      compare_dirs(origin.path(), destiny, setup)?;
    } else {
      compare_file(origin.path(), destiny, setup)?;
    }
  }
  Ok(())
}

fn compare_file(side_a: PathBuf, side_b: PathBuf, setup: &Setup) -> RubxResult<()> {
  if !side_a.exists() {
    println!(
      "Comparing file: '{}' with '{}': side_a not exists.",
      side_a.display(),
      side_b.display()
    );
    return Ok(());
  }
  if !side_a.is_file() {
    println!(
      "Comparing file: '{}' with '{}': side_a is not a file.",
      side_a.display(),
      side_b.display()
    );
    return Ok(());
  }
  if !side_b.exists() {
    println!(
      "Comparing file: '{}' with '{}': side_b not exists.",
      side_a.display(),
      side_b.display()
    );
    return Ok(());
  }
  if !side_b.is_file() {
    println!(
      "Comparing file: '{}' with '{}': side_b is not a file.",
      side_a.display(),
      side_b.display()
    );
    println!("");
    return Ok(());
  }
  let side_a_meta = side_a.metadata()?;
  let side_b_meta = side_b.metadata()?;
  let mut diff_size = false;
  let mut diff_created: Option<bool> = None;
  let mut diff_modified: Option<bool> = None;
  let mut diff_accessed: Option<bool> = None;
  if setup.check_all || setup.check_size {
    let side_a_size = side_a_meta.len();
    let side_b_size = side_b_meta.len();
    if side_a_size != side_b_size {
      diff_size = true;
    }
  }
  if setup.check_all || setup.check_created {
    if let Ok(side_a_time) = side_a_meta.created() {
      if let Ok(side_b_time) = side_b_meta.created() {
        if side_a_time != side_b_time {
          diff_created = Some(true);
        } else {
          diff_accessed = Some(false);
        }
      }
    }
  }
  if setup.check_all || setup.check_modified {
    if let Ok(side_a_time) = side_a_meta.modified() {
      if let Ok(side_b_time) = side_b_meta.modified() {
        if side_a_time != side_b_time {
          diff_modified = Some(true);
        } else {
          diff_accessed = Some(false);
        }
      }
    }
  }
  if setup.check_all || setup.check_accessed {
    if let Ok(side_a_time) = side_a_meta.accessed() {
      if let Ok(side_b_time) = side_b_meta.accessed() {
        if side_a_time != side_b_time {
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
    side_a.display(),
    side_b.display()
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
  let should_print = !setup.only_diffs || has_diffs;
  if should_print {
    println!("{}", result);
  }
  Ok(())
}
