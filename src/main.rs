mod fs_handling;
mod regex_validation;

use clap::Parser;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

#[cfg(target_family = "unix")]
const CONFIG_FILE: &str = "config.txt";

#[cfg(target_family = "windows")]
const CONFIG_FILE: &str = "não sei ainda";

#[cfg(target_family = "unix")]
fn initialize_git() -> () {
     
}

#[cfg(target_family = "windows")]
fn initialize_git() -> () {
     
}

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

enum ConfigParserState {
    SearchingForConfigBlock,
    ParsingConfigBlock,
}

fn main() -> () {
    let (config_file_handler, created_config_file) : (File, bool) = fs_handling::config_file_opener(CONFIG_FILE);
    
    let current_user_args = UserCLIArgsFormat::parse(); 
    
    // Initialize git repository (default )
    if current_user_args.git_repository_link != None {
         
    }

    // Check if the program had to create the confuration file that should be there
    // NOTE: In the future, the program should create the default configuration file and not just
    // an empty one
    if created_config_file {
        println!("No configuration file was detected, so we created a new one in {}.\nHowever, the file is empty and so you have to write your configurations. In the meantime, the program cannot execute because it does not know what to do.", CONFIG_FILE); 
        return;
    }

    // Create main folder
    fs_handling::create_folder(&current_user_args.project_name);
      
    // Start reading the configuration file 
    let mut line_reader: BufReader<File> = BufReader::new(config_file_handler);
    let mut current_line: String = String::new();
    
    // Flags and other info of the process of reading the file
    let mut parser_config_state: ConfigParserState = ConfigParserState::SearchingForConfigBlock;
    let mut current_line_number: i32 = 0;
    
    while line_reader.read_line(&mut current_line).unwrap() != 0 {
        
        current_line_number += 1;
        let current_trimmed_line: &str = current_line.trim(); 
        
        match parser_config_state {
            ConfigParserState::SearchingForConfigBlock => {
                if regex_validation::is_config_name(&current_line) && &current_line[1..current_user_args.config_name.len() + 1] == current_user_args.config_name {
                    parser_config_state = ConfigParserState::ParsingConfigBlock; 
                }
            },
            ConfigParserState::ParsingConfigBlock => {
                if regex_validation::is_create_file_line(&current_trimmed_line) {

                }
                else if regex_validation::is_create_folder_line(&current_trimmed_line) {
                    let current_command_args: &str = &current_trimmed_line[14..];
                    let v: Vec<&str> = current_command_args.split(",").collect();
                    let (mut create_folder_args, mut path_to_create): (&str, String) = ("", current_user_args.project_name.clone());
                   
                    for i in 0..v.len() {
                        create_folder_args = v[i];
                        let mut trimmed_folder_args: &str = create_folder_args.trim();
                        
                        if i == v.len() - 1 {
                            trimmed_folder_args = &trimmed_folder_args[..trimmed_folder_args.len() - 1];
                        }
                        
                        path_to_create.push('/'); path_to_create.push_str(trimmed_folder_args);
                        fs_handling::create_folder(&path_to_create);

                        path_to_create.clear(); path_to_create = current_user_args.project_name.clone();
                    }
                }
                else if regex_validation::is_documentation_specifier(&current_trimmed_line) {
                    
                } else if regex_validation::is_comment(&current_trimmed_line) {
                    continue; 
                }
                else if regex_validation::is_config_name(&current_trimmed_line) {
                     break;
                } else {
                     println!("You have a problem in line {} with content: {} in the configuration file located at {}", current_line_number, current_line, CONFIG_FILE);
                     return;
                }
                
            },
        }

        current_line.clear();
    } 

}
