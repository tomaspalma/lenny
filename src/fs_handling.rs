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

pub fn create_folder(folder_path: &PathBuf) -> () {
    //println!("Creating directory { } ...", folder_path);
    if folder_path.is_dir() || folder_path.is_file() {
        println!("A folder with the name of your project name: {} already exists. Select the option you want to do:", &folder_path.display());
        println!("1. Override folder and write to it anyway");
        println!("2. Deal with it manually either by deleting the folder or even changing the project name argument while calling lenny for example");

        let mut option: String = String::new();
        while option != "1" && option != "2" {
            option.clear();
            std::io::stdin()
                .read_line(&mut option)
                .expect("Invalid input!");
            option = option.trim().to_string();
        }

        if option == "1" {
            create_dir(folder_path).unwrap();
        } else {
            return;
        }
    } else {
        //Create folder
        create_dir_all(folder_path).unwrap();
    }
}

pub fn create_empty_file(file_path: &PathBuf) -> () {
    //println!("Creating file { } ...", file_path);
    File::create(file_path).unwrap();
}

pub fn create_non_empty_file(file_path: &PathBuf, text_to_write: &String) -> () {
    let mut file_handler: File = File::create(file_path).unwrap();
    file_handler.write_all(text_to_write.as_bytes()).unwrap();
}
