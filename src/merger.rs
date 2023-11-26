use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Read, Write};
use std::path::PathBuf;

use eyre::{eyre, Result, WrapErr};
use indicatif::ProgressBar;
use log::debug;

use crate::paths;

const MEGABYTE: usize = 1024 * 1024;

pub struct MergerUnmerger {
    prepend_line: String,
    append_line: String,
}

impl MergerUnmerger {
    pub fn new(prepend_line: String, append_line: String) -> Result<Self> {
        if prepend_line == append_line {
            return Err(eyre!("prepend and append lines can not be the same!"));
        }

        Ok(MergerUnmerger {
            prepend_line,
            append_line,
        })
    }

    pub fn can_merge(&self, paths: &Vec<PathBuf>) -> Result<()> {
        let mut errors = Vec::<String>::new();
        for path in paths {
            let path_str = String::from(path.to_str().unwrap_or(""));

            // if file size is too big
            let file_size = path.metadata().unwrap().len() as usize;
            if file_size > MEGABYTE {
                errors.push(format!("File #{} is too big (>1mb).", path_str));
                continue;
            }

            // if file size is not utf-8
            let mut file = File::open(path).unwrap();
            let mut buffer = String::with_capacity(file_size);
            debug!("Reading file {} into memory", path_str);
            if file.read_to_string(&mut buffer).is_err() {
                errors.push(format!("File #{} could not be read.", path_str));
                continue;
            }

            debug!("Checking all lines of file {} for prepend/append", path_str);
            // if prepend or append exists in the file
            let buf_reader = BufReader::new(Cursor::new(buffer));
            for (line_res, line_no) in buf_reader.lines().zip(1..) {
                if let Ok(line) = line_res {
                    if line.starts_with(&self.prepend_line) {
                        errors.push(format!(
                            "Line #{} in file #{} starts with prepend line",
                            line_no, path_str
                        ));
                    } else if line == self.append_line {
                        errors.push(format!(
                            "Line #{} in file #{} is the same as the append line",
                            line_no, path_str
                        ));
                    }
                } else if let Err(e) = line_res {
                    errors.push(format!(
                        "Line #{} in file #{} was not readable: {}",
                        line_no, path_str, e
                    ));
                    continue;
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(eyre!(errors.join("\n")))
        }
    }

    pub fn merge(&self, paths: &Vec<PathBuf>, output: &PathBuf) -> Result<()> {
        let output_str = String::from(output.to_str().unwrap_or(""));
        let mut output_file = File::create(output)
            .wrap_err(format!("Could not create output file: {}", output_str))?;

        let progress = ProgressBar::new(paths.len() as u64);
        for path in paths {
            let path_str = String::from(path.to_str().unwrap_or(""));
            debug!("Adding file {} to output", path_str);

            output_file
                .write_fmt(format_args!("{} {}\n", self.prepend_line, path_str))
                .wrap_err("Could not write prepend line to output file.")?;

            let file_size = path.metadata().unwrap().len() as usize;
            let mut file = File::open(path).unwrap();
            let mut buffer = String::with_capacity(file_size);
            file.read_to_string(&mut buffer)
                .wrap_err(format!("Could not read file {} into memory.", path_str))?;

            output_file
                .write_all(buffer.as_bytes())
                .wrap_err(format!("Could not write file {} to output.", path_str))?;

            output_file
                .write_fmt(format_args!("{}\n", self.append_line))
                .wrap_err("Could not write append line to output file.")?;

            progress.inc(1);
        }

        progress.finish_and_clear();
        Ok(())
    }

    pub fn unmerge(&self, merged: &PathBuf) -> Result<()> {
        let mut file = RefCell::new(None::<File>);

        let progress = ProgressBar::new_spinner();
        for (line_res, line_no) in BufReader::new(File::open(merged)?).lines().zip(1..) {
            let line = line_res?;
            if line.starts_with(&self.prepend_line) {
                if file.borrow().is_some() {
                    return Err(eyre!(
                        "Found prepend_line before append_line at line #{}",
                        line_no
                    ));
                }

                let path =
                    PathBuf::from(&line.as_str()[(&self.prepend_line.len() + 1)..].to_string());
                if path.exists() {
                    fs::remove_file(path.clone())?;
                }
                paths::mkdir(&path)?;
                *file.get_mut() = Some(File::create(path)?);
                continue;
            } else {
                if file.borrow().is_some() {
                    if line != self.append_line {
                        file.borrow_mut()
                            .as_mut()
                            .unwrap()
                            .write_fmt(format_args!("{}\n", line))
                            .wrap_err("Could not write append line to output file.")?;
                    } else {
                        *file.get_mut() = None;
                        progress.inc(1);
                    }
                } else {
                    return Err(eyre!(
                        "Found append_line/regular_line with no prior prepend_line at line #{}",
                        line_no
                    ));
                }
            }
        }

        progress.finish_and_clear();
        Ok(())
    }
}
