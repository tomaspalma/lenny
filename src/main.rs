mod fs_handling;
mod regex_validation;

use clap::Parser;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

const CONFIG_FILE_NAME: &str = "lenny/";

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
    #[clap(short='f', long="mainfoldername", value_parser)]
    project_name: String,
    
    /// Links with already created repository
    #[clap(short='g', long="git", value_parser)]
    git_repository_link: Option<String>
}

#[derive(Debug)]
enum ConfigParserState {
    SearchingForConfigBlock,
    ParsingConfigBlock,
}

fn main() -> () {
    
    let available_commands: Vec<&str> = vec!["CreateFiles", "CreateFolders", "Documentation"];

    let mut config_file_full_dir: String = String::new();

    // Define parent home directory in config file location
    if cfg!(unix) {
        match std::env::var_os("HOME") {
             Some(value) => config_file_full_dir = format!("{}{}{}", value.into_string().unwrap(), String::from("/.config/"), CONFIG_FILE_NAME),
             None => panic!("Environment variable $HOME is not set. Please set it correctly."),
        }
    }

    // See if the configuration file directory is already created
    fs_handling::create_folder(&config_file_full_dir);
    config_file_full_dir.push_str(&"config.txt");
    
    let (config_file_handler, created_config_file) : (File, bool) = fs_handling::config_file_opener(&config_file_full_dir);
   
    let current_user_args = UserCLIArgsFormat::parse(); 
    
    // Initialize git repository (default )
    if current_user_args.git_repository_link != None {
         
    }

    // Check if the program had to create the confuration file that should be there
    // NOTE: In the future, the program should create the default configuration file and not just
    // an empty one
    if created_config_file {
        println!("No configuration file was detected, so we created a new one in {}.\nHowever, the file is empty and so you have to write your configurations. In the meantime, the program cannot execute because it does not know what to do.", config_file_full_dir); 
        return;
    }

    // Create main folder
    fs_handling::create_folder(&current_user_args.project_name);
      
    // Start reading the configuration file 
    let mut line_reader: BufReader<File> = BufReader::new(config_file_handler);
    let mut current_line: String = String::new();
    
    // Flags and other info of the process of reading the file
    let mut parser_config_state: ConfigParserState = ConfigParserState::SearchingForConfigBlock;
    let mut found_documentation_config: bool = false;
    let mut current_line_number: i32 = 0;

    let mut global_folder_parent: String = current_user_args.project_name.clone();
    
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
                if regex_validation::is_create_empty_file_line(&current_trimmed_line) {
                    let part_of_create_file_command_args: &str = &current_trimmed_line[12..];
                    let args: Vec<&str> = part_of_create_file_command_args.split(",").collect(); let args_len = args.len();
                    let mut create_file_args: &str = ""; let mut trimmed_file_args: &str = "";

                    for i in 0..args_len {
                        create_file_args = args[i];
                        trimmed_file_args = create_file_args.trim();

                        if i == args_len - 1 {
                             trimmed_file_args = &trimmed_file_args[..trimmed_file_args.len() - 1];
                        }

                        global_folder_parent.push('/'); global_folder_parent.push_str(trimmed_file_args);
                        fs_handling::create_empty_file(&global_folder_parent);

                        global_folder_parent = current_user_args.project_name.clone();
                    }

                } else if regex_validation::is_create_folder_line(&current_trimmed_line) {
                    let part_of_create_folder_command_args: &str = &current_trimmed_line[14..];
                    let args: Vec<&str> = part_of_create_folder_command_args.split(",").collect(); let args_len = args.len();
                    let mut create_folder_args: &str= ""; let mut trimmed_folder_args: &str = ""; 
                   
                    for i in 0..args_len {
                        create_folder_args = args[i];
                        trimmed_folder_args = create_folder_args.trim();
                        
                        if i == args_len - 1 {
                            trimmed_folder_args = &trimmed_folder_args[..trimmed_folder_args.len() - 1];
                        }
                        
                        global_folder_parent.push('/'); global_folder_parent.push_str(trimmed_folder_args);
                        fs_handling::create_folder(&global_folder_parent);

                        global_folder_parent = current_user_args.project_name.clone();
                    }

                } else if regex_validation::is_documentation_specifier(&current_trimmed_line) {
                    found_documentation_config = true; 
                } else if regex_validation::is_comment(&current_trimmed_line) {
                    current_line.clear();
                    continue;
                }
                else if regex_validation::is_config_name(&current_trimmed_line) {
                     break;
                } else {
                    // Try to tell the user specifically where the error is at
                     println!("You have a problem in line {} with content: {} in the configuration file located at {}", current_line_number, current_line, config_file_full_dir);
                     return;
                }
                
            },
        }

        current_line.clear();
    }

    if let ConfigParserState::SearchingForConfigBlock = parser_config_state {
         println!("The configuration name you specified in the program parameters was not found in the file. Please, check for spelling mistakes or other mistakes.");
    } else if current_user_args.generate_documentation && !found_documentation_config {
         println!("In the program parameters you specified you wanted to use a programming documentation engine. However, in the configuration file in the block of the config you specified, there is no definition of the documentation engine. Add this to the config file: Documentation([name_of_the_engine])");
    } else {
         println!("Folder structure successfully created.");
    }
}
