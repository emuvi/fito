use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::Arc;
use crate::FitoError;


pub fn make(speed: usize, input: &str, output: &str) -> Result<(), FitoError> {
    let input = Path::new(input);
    let output = Path::new(output);
    if input.is_file() {
        copy_file(input, output)?;
    } else {
        make_dir(speed, input, output);
    }
    Ok(())
}

type CopyLink = (PathBuf, PathBuf);

fn make_dir(speed: usize, input: impl AsRef<Path>, output: impl AsRef<Path>) -> Result<(), FitoError> {
    let pool: Arc<Mutex<Vec<CopyLink>>> = Arc::new(Mutex::new(Vec::new()));
    let done: Arc<RwLock<bool>> = Arc::new(RwLock::new(false));
    for _ in 0..speed {
        let pool_arc = pool.clone();
        let done_arc = done.clone();
        let tst = std::thread::spawn(move || {
            loop {
                let item = {
                    let lock = pool_arc.lock().unwrap();
                    lock.pop()
                };
                if *done_arc.read().unwrap() {
                    break;
                }
            }
        });
    }
    Ok(())
}

fn copy_directory(
    queue: &SegQueue<(PathBuf, PathBuf)>,
    origin: impl AsRef<Path>,
    destiny: impl AsRef<Path>,
) -> Result<(), FitoError> {
    fs::create_dir_all(&destiny)?;
    for entry in fs::read_dir(origin)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_directory(
                &queue,
                entry.path(),
                destiny.as_ref().join(entry.file_name()),
            )?;
        } else {
            let from = entry.path();
            let to = destiny.as_ref().join(entry.file_name());
            queue.push((from, to));
        }
    }
    Ok(())
}

fn copy_file(origin: impl AsRef<Path>, destiny: impl AsRef<Path>) -> Result<(), FitoError> {
    if let Some(parent) = destiny.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(origin, destiny)?;
    Ok(())
}

// move || {
//     println!(
//         "Starting to copy from '{}' to '{}'.",
//         from.display(),
//         to.display()
//     );
//     if let Err(e) = fs::copy(&from, &to) {
//         eprintln!(
//             "Could not copy from '{}' to '{}' with error:\n{}",
//             from.display(),
//             to.display(),
//             e
//         );
//     } else {
//         println!(
//             "Finished to copy from '{}' to '{}'.",
//             from.display(),
//             to.display()
//         );
//     }
// }
