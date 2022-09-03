use std::fs::{create_dir, create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

pub fn open_file(file_path: &PathBuf) -> File {
    let file_handler = File::open(file_path).unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            let file_creation_handler = File::create(file_path).unwrap();

            file_creation_handler
        } else {
            panic!("Error opening file: { }", error);
        }
    });

    file_handler
}

pub fn create_non_empty_file(file_path: &PathBuf, text_to_write: &String) -> () {
    let mut file_handler: File = File::create(file_path).unwrap();
    file_handler.write_all(text_to_write.as_bytes()).unwrap();
}
