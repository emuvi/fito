use regex::Regex;
use rubx::rux_paths;
use rubx::RubxResult;

use std::path::PathBuf;

pub struct Setup {
  pub verbose: bool,
  pub all_extensions: bool,
  pub by_name: Option<Regex>,
  pub by_extensions: Option<Vec<String>>,
}

impl Setup {
  fn println(&self, message: &str) {
    if self.verbose {
      println!("{}", message);
    }
  }
}

pub fn start(from: PathBuf, setup: Setup) -> RubxResult<()> {
  if setup.all_extensions {
    setup.println("Finding extensions:");
    find_all_extensions(&from)?;
  }
  if setup.by_name.is_some() {
    setup.println("Finding by name:");
    find_by_name(&from, &setup)?;
  }
  if setup.by_extensions.is_some() {
    setup.println("Finding by name:");
    find_by_extensions(&from, &setup)?;
  }
  Ok(())
}

fn find_all_extensions(from: &PathBuf) -> RubxResult<()> {
  let mut found: Vec<String> = vec![];
  let mut act = |path: &PathBuf| {
    if path.is_dir() {
      return;
    }
    if let Some(ext) = path.extension() {
      if let Some(ext) = ext.to_str() {
        let ext = ext.to_string();
        if !found.contains(&ext) {
          println!("{}", ext);
          found.push(ext);
        }
      }
    }
  };
  rux_paths::traverse(from, &mut act)
}

fn find_by_name(from: &PathBuf, setup: &Setup) -> RubxResult<()> {
  let mut act = |path: &PathBuf| {
    if let Some(name) = path.file_name() {
      if let Some(name) = name.to_str() {
        let regex = setup.by_name.as_ref().unwrap();
        if regex.is_match(name) {
          println!("{}", from.display());
        }
      }
    }
  };
  rux_paths::traverse(from, &mut act)
}

fn find_by_extensions(from: &PathBuf, setup: &Setup) -> RubxResult<()> {
  let extensions = setup.by_extensions.as_ref().unwrap();
  let mut act = |path: &PathBuf| {
    if path.is_dir() {
      return;
    }
    if let Some(ext) = path.extension() {
      if let Some(ext) = ext.to_str() {
        let ext = ext.to_string();
        if extensions.contains(&ext) {
          println!("{}", path.to_string_lossy());
        }
      }
    }
  };
  rux_paths::traverse(from, &mut act)
}
