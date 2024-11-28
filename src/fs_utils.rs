use std::fs;
use std::path::Path;

use anyhow::{bail, Result};

/// Recursively walks a given directory path returning all decendant paths
pub fn walk_directory(path: &Path) -> Result<Vec<Box<Path>>> {
    if !path.exists() {
        bail!("Path '{}' does not exist", path.display())
    }

    if !path.is_dir() {
        bail!("Path '{}' is not a directory", path.display())
    }

    let mut result = Vec::new();

    if let Ok(rd) = fs::read_dir(path) {
        // Iterate over each item in the directory
        for entry in rd {
            if entry.is_err() {
                continue;
            }
            let entry = entry.unwrap();
            let entry_path = entry.path();

            if entry_path.is_dir() {
                let mut x = walk_directory(entry_path.as_path())?;
                result.append(&mut x);
                continue;
            }

            if entry_path.is_file() {
                let fo = entry_path.into_boxed_path();
                result.push(fo);
            }
        }
    } else {
        return Ok(vec![]);
    }

    Ok(result)
}
