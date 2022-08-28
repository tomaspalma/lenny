mod fs_handling;
mod regex_validation;

use clap::Parser;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

#[derive(Parser, Debug)]
struct UserCLIArgsFormat {
    /// Generate documentation configuration files 
    #[clap(short='d', long="docs", action = clap::ArgAction::SetTrue)]
    generate_documentation: bool,
    
    /// Name of the config title in the config file
    #[clap(short='c', long="config", value_parser)]
    config_name: String,
    
    /// Name of the main folder of the project
    project_name: String,
    
    /// Links with already created repository
    #[clap(short='g', long="git", value_parser)]
    git_repository_link: Option<String>
}

fn main() -> () {
    let config_file_path: &str = "~/.config/lenny/config.txt";
    let (config_file, created_config_file) : (File, bool) = fs_handling::config_file_opener("config.txt");
    
    let current_user_args = UserCLIArgsFormat::parse(); 
     
    // Check if the program had to create the confuration file that should be there
    // NOTE: In the future, the program should create the default configuration file and not just
    // an empty one
    if created_config_file {
        println!("No configuration file was detected, so we created a new one in {}.\nHowever, the file is empty and so you have to write your configurations. In the meantime, the program cannot execute because it does not know what to do.", config_file_path); 
        return;
    }
    
    // Start reading the configuration file 
    let mut line_reader: BufReader<File> = BufReader::new(config_file);
    let mut current_line: String = String::new();
    
    // Flags of the process of reading the file
    let mut found_config_location = false;

    while line_reader.read_line(&mut current_line).unwrap() != 0 {
        
        println!("{ }", regex_validation::is_config_name(&current_line));

        if !found_config_location {
           if regex_validation::is_config_name(&current_line) && &current_line[1..current_user_args.config_name.len() + 1] == current_user_args.config_name {
               found_config_location = true; 
               print!("found it");
           }
        }

        current_line.clear();
    }

}
