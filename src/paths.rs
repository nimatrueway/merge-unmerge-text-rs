use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use eyre::{Result, WrapErr};

pub fn mkdir(f: &PathBuf) -> Result<()> {
    if let Some(parent_dir) = f.parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir).wrap_err(format!(
                "Could not create directory for file: {}",
                f.to_str().unwrap_or("")
            ))
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

pub fn expand_and_merge(globs: Vec<String>) -> Result<Vec<PathBuf>> {
    let include_files: Vec<PathBuf> = globs
        .iter()
        .filter(|arg| !arg.starts_with("!"))
        .flat_map(|arg| {
            glob::glob(&arg)
                .wrap_err(format!("Could not parse glob {}", arg))
                .unwrap()
        })
        .map(|path| path.unwrap())
        .collect();

    let exclude_files: HashSet<PathBuf> = globs
        .iter()
        .filter(|arg| arg.starts_with("!"))
        .flat_map(|arg| {
            glob::glob(arg[1..].as_ref())
                .wrap_err(format!("Could not parse glob {}", arg))
                .unwrap()
        })
        .map(|path| path.unwrap())
        .collect();

    Ok(include_files
        .into_iter()
        .filter(|path| !exclude_files.contains(path))
        .collect())
}
