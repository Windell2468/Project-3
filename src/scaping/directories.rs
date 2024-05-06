// directories.rs or directories/mod.rs

use std::fs;
use std::path::Path;
use std::io;

// Function to list all files in a directory
pub fn list_files_in_directory(dir_path: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let file_name = entry.file_name().into_string().unwrap(); // Convert OsString to String
        files.push(file_name);
    }
    Ok(files)
}

// Function to check if a directory exists
pub fn directory_exists(dir_path: &str) -> bool {
    Path::new(dir_path).is_dir()
}
