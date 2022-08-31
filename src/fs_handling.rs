use std::{fs::File, io::Write};

pub fn config_file_opener(file: &str) -> (File, bool) {
    let mut created_config_file = false;
    
    let file_handler = File::options().open(file).unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            let file_creation_handler = File::create(file).unwrap();

            created_config_file = true;
            return file_creation_handler;
        } else {
            panic!("Error opening file: { }", error);
        }
    });

    (file_handler, created_config_file)
}

pub fn create_folder(folder_path: &String) -> () { 
    //println!("Creating directory { } ...", folder_path);
    std::fs::create_dir_all(folder_path).unwrap_or_else(|error| {
        println!("Couldn't create directory: {}", error); 
    });     
}

pub fn create_empty_file(file_path: &String) -> () {
    //println!("Creating file { } ...", file_path); 
    File::create(file_path).unwrap();
}

pub fn create_non_empty_file(file_path: &String, text_to_write: &String) -> () {
    let mut file_handler: File = File::create(file_path).unwrap(); 
    file_handler.write_all(text_to_write.as_bytes()).unwrap();
}
