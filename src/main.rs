mod fs_handling;
mod regex_validation;

use clap::Parser;
use std::io::BufReader;
use std::fs::File;

#[derive(Parser, Debug)]
struct UserCLIArgs {
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
    git_repository_link: Option<String>,
}

fn main() -> () {
    let config_file_path = "~/.config/lenny/config.txt";
    let user_args = UserCLIArgs::parse(); 
     
    let (file, created_config_file) : (File, bool) = fs_handling::config_file_opener("config.txt");
    
    // Check if the program had to create the confuration file that should be there
    // NOTE: In the future, the program should create the default configuration file and not just
    // an empty one
    if created_config_file {
        println!("No configuration file was detected, so we created a new one in {}.\nHowever, the file is empty and so you have to write your configurations. In the meantime, the program cannot execute because it does not know what to do.", config_file_path); 
        return;
    }
    
    // Start reading the configuration file 

}
