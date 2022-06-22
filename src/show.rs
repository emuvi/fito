use chrono::offset::Utc;
use chrono::DateTime;
use rubx::RubxResult;

use std::path::PathBuf;

pub fn info(address: PathBuf) -> RubxResult<()> {
  if address.is_dir() {
    info_of_dir(address)
  } else {
    info_of_file(address)
  }
}

fn info_of_dir(address: PathBuf) -> RubxResult<()> {
  for inside in address.read_dir()? {
    let inside = inside?;
    let name = inside.file_name();
    println!("{}", name.to_string_lossy());
  }
  Ok(())
}

fn info_of_file(address: PathBuf) -> RubxResult<()> {
  let address = std::fs::canonicalize(address)?;
  println!("{}", address.to_string_lossy());
  let metadata = std::fs::metadata(address)?;
  println!("Size: {}", metadata.len());
  if let Ok(created) = metadata.created() {
    let created: DateTime<Utc> = created.into();
    println!("Created: {}", created);
  }
  if let Ok(modified) = metadata.modified() {
    let modified: DateTime<Utc> = modified.into();
    println!("Modified: {}", modified);
  }
  if let Ok(accessed) = metadata.accessed() {
    let accessed: DateTime<Utc> = accessed.into();
    println!("Accessed: {}", accessed);
  }
  Ok(())
}
