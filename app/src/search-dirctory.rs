use std::error::Error;
use std::fs;
use std::path;

pub fn search_dir(path: &str) -> Result<Vec<path::PathBuf>, Box<dyn Error>> {
    let dir = fs::read_dir(path)?;
    let mut files: Vec<path::PathBuf> = Vec::new();
    for item in dir.into_iter() {
        files.push(item?.path());
    }
    Ok(files)
}