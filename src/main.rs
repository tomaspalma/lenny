mod fs_handling;
mod strings_validation;

use clap::Parser;
use std::fs::{create_dir_all, File};
use std::io::BufReader;
use std::io::{stdin, BufRead};
use std::path::PathBuf;

#[cfg(target_family = "unix")]
fn initialize_git() -> () {}

#[cfg(target_family = "windows")]
fn initialize_git() -> () {}

#[derive(Parser, Debug)]
struct UserCLIArgsFormat {
    /// (OPTIONAL) Generate documentation configuration files
    #[clap(short='d', long="docs", action = clap::ArgAction::SetTrue)]
    generate_documentation: bool,

    /// (OPTIONAL) Activate search in [config_name].txt instead of config.txt
    #[clap(short='a', long="--alternativecfg", action = clap::ArgAction::SetTrue)]
    alternative_cfg: bool,

    /// Name of the config title in the config file
    #[clap(short = 'c', long = "config", value_parser)]
    config_name: String,

    /// Name of the main folder of the project
    #[clap(short = 'n', long = "name", value_parser)]
    project_name: PathBuf,

    /// (OPTIONAL) Links with already created repository
    #[clap(short = 'g', long = "git", value_parser)]
    git_repository_link: Option<String>,
}

#[derive(Debug)]
enum ConfigParserState {
    SearchingForConfigBlock,
    ParsingConfigFuncs,
}

#[derive(Debug)]
enum ConfigFunctions {
    CreateEmptyFiles(PathBuf),
    CreateFolder(PathBuf),
    CreateNonEmptyFile(PathBuf, String),
}

fn main() -> () {
    let current_user_args = UserCLIArgsFormat::parse();

    // Initialize git repository (default )
    if current_user_args.git_repository_link != None {}

    let available_commands: Vec<&str> =
        vec!["CreateEmptyFiles", "CreateFolders", "CreateNonEmptyFile"];
    let (create_files_command_len, create_folders_command_len, write_to_file_command_len): (
        usize,
        usize,
        usize,
    ) = (
        available_commands[0].len(),
        available_commands[1].len(),
        available_commands[2].len(),
    );

    let (mut config_folder_path, mut config_file_path): (PathBuf, PathBuf) =
        (PathBuf::new(), PathBuf::new());

    // Define parent home directory in config file location
    if cfg!(unix) {
        match std::env::var_os("HOME") {
            Some(value) => {
                config_folder_path.push(&value);
                config_folder_path.push(".config");
                config_folder_path.push("lenny");
            }
            None => {
                println!("Environment variable $HOME is not set. Please set it correctly.");
                return;
            }
        }
    }

    config_file_path.push(&config_folder_path);
    if current_user_args.alternative_cfg {
        config_file_path.push(&current_user_args.project_name);
    } else {
        config_file_path.push("config");
    }
    config_file_path.set_extension("txt");

    if !&config_file_path.is_file() {
        if !&config_folder_path.is_dir() {
            create_dir_all(&config_folder_path).unwrap();
        }
        File::create(&config_file_path).unwrap();
        println!(
            "There was no configuration file available, so we created one at {}. Since no config file was created, it's empty and there's nothing to parse. Please write to the config file",
            config_folder_path.display()
        );
        return;
    }

    let config_file_handler = fs_handling::open_file(&config_file_path);

    // Check if the program had to create the confuration file that should be there
    // NOTE: In the future, the program should create the default configuration file and not just
    // an empty one
    /*if created_config_file {
        println!("No configuration file was detected, so we created a new one in {}.\nHowever, the file is empty and so you have to write your configurations. In the meantime, the program cannot execute because it does not know what to do.", config_file_full_dir.display());
        return;
    } */

    // See if the project name file directory is already created, so we can catch errors that could
    // override a directory already created in the user' s system that was not supposed to because
    // of a spelling mistake of the user, for example. However, they could have been creating a
    // folder with lenny but they had an error with the config file. After they fixed it, they run
    // lenny with the same folder name, they can delete it still but if they run lenny with that
    // folder name again we should ask what they want to do. If they want to override the folder,
    // or do nothing, eliminate that folder and then run lenny with that name or run lenny with
    // another name ,for example

    // Start reading the configuration file
    let mut line_reader: BufReader<File> = BufReader::new(config_file_handler);
    let mut current_line: String = String::new();

    // Flags and other info of the process of reading the file
    let mut parser_config_state: ConfigParserState;
    if current_user_args.alternative_cfg {
        parser_config_state = ConfigParserState::ParsingConfigFuncs;
    } else {
        parser_config_state = ConfigParserState::SearchingForConfigBlock;
    }

    let mut found_documentation_config: bool = false;
    let mut current_line_number: i32 = 0;

    let mut global_folder_parent: PathBuf = current_user_args.project_name.clone();

    let mut commands_to_execute_queue: Vec<ConfigFunctions> = Vec::new();

    if PathBuf::from(&current_user_args.project_name).is_dir() {
        println!("There's already a folder with the project name created in the current directory. Choose one of the options to proceed:");
        println!("1. Merge already created directory with new one");
        println!("2. Manually solve the problem, either by changing the project name argument when calling the program or deleting the already existing folder");

        let mut option: String = String::new();
        let input_reader = stdin();
        while option != "1" && option != "2" {
            input_reader.read_line(&mut option).unwrap();
            option = option.trim().to_string();
        }

        if option == "2" {
            return;
        }
    }

    // Parse config file
    while line_reader.read_line(&mut current_line).unwrap() != 0 {
        current_line_number += 1;
        let current_trimmed_line: &str = current_line.trim();

        match parser_config_state {
            ConfigParserState::SearchingForConfigBlock => {
                if strings_validation::is_config_name(&current_line)
                    && &current_line[1..current_user_args.config_name.len() + 1]
                        == current_user_args.config_name
                {
                    parser_config_state = ConfigParserState::ParsingConfigFuncs;
                }
            }

            ConfigParserState::ParsingConfigFuncs => {
                if strings_validation::is_create_folder_line(&current_trimmed_line) {
                    let part_of_create_folder_command_args: &str =
                        &current_trimmed_line[create_folders_command_len + 1..];
                    let args: Vec<&str> = part_of_create_folder_command_args.split(",").collect();
                    let args_len = args.len();
                    let mut create_folder_args: &str;
                    let mut trimmed_folder_args: &str;

                    for i in 0..args_len {
                        create_folder_args = args[i];
                        trimmed_folder_args = create_folder_args.trim();

                        if i == args_len - 1 {
                            trimmed_folder_args =
                                &trimmed_folder_args[..trimmed_folder_args.len() - 1];
                        }

                        global_folder_parent.push(trimmed_folder_args);
                        //fs_handling::create_folder(&global_folder_parent);
                        commands_to_execute_queue
                            .push(ConfigFunctions::CreateFolder(global_folder_parent));

                        global_folder_parent = current_user_args.project_name.clone();
                    }
                } else if strings_validation::is_create_empty_file_line(&current_trimmed_line) {
                    let part_of_create_file_args: &str =
                        &current_trimmed_line[create_files_command_len + 1..];
                    let args: Vec<&str> = part_of_create_file_args.split(",").collect();
                    let args_len = args.len();
                    let mut create_file_args: &str;
                    let mut trimmed_file_args: &str;

                    for i in 0..args_len {
                        create_file_args = args[i];
                        trimmed_file_args = create_file_args.trim();

                        if i == args_len - 1 {
                            trimmed_file_args = &trimmed_file_args[..trimmed_file_args.len() - 1];
                        }

                        global_folder_parent.push(trimmed_file_args);
                        commands_to_execute_queue
                            .push(ConfigFunctions::CreateEmptyFiles(global_folder_parent));

                        global_folder_parent = current_user_args.project_name.clone();
                    }
                } else if strings_validation::is_write_to_file_line(&current_trimmed_line) {
                    let part_of_write_to_file_args: &str =
                        &current_trimmed_line[write_to_file_command_len + 1..];
                    let mut brackets_stack: Vec<char> = vec!['('];

                    let first_line_args: Vec<&str> =
                        part_of_write_to_file_args.splitn(2, ",").collect();

                    global_folder_parent.push(first_line_args.get(0).unwrap().to_string());

                    let first_part_of_text: &str = first_line_args.get(1).unwrap();

                    let mut text_to_write_holder: String = String::new();

                    // Firstly, we write to the string we're going to write to the file the chars
                    // from the first line of the config.txt arg
                    // check_create_file_lines(&current_line, &mut brackers_stack, &mut String text_to_write_holder);
                    for character in first_part_of_text.chars() {
                        text_to_write_holder.push(character);
                        if character == '(' {
                            brackets_stack.push(character);
                        } else if character == ')' {
                            brackets_stack.pop();
                        }
                    }

                    current_line.clear();

                    while !brackets_stack.is_empty() {
                        if line_reader.read_line(&mut current_line).unwrap() == 0 {
                            print!("Command CreateNonEmptyFiles' parentheses are not correctly closed.");
                            return;
                        }

                        for character in current_line.chars() {
                            if character == '(' {
                                brackets_stack.push(character);
                            } else if character == ')' {
                                brackets_stack.pop();
                            }

                            if !brackets_stack.is_empty() {
                                text_to_write_holder.push(character);
                            }
                        }

                        current_line.clear();
                    }

                    commands_to_execute_queue.push(ConfigFunctions::CreateNonEmptyFile(
                        global_folder_parent,
                        text_to_write_holder,
                    ));
                    global_folder_parent = current_user_args.project_name.clone();
                } else if strings_validation::is_documentation_specifier(&current_trimmed_line) {
                    found_documentation_config = true;
                } else if strings_validation::is_comment(&current_trimmed_line)
                    || current_trimmed_line == ""
                {
                    current_line.clear();
                    continue;
                } else if strings_validation::is_config_name(&current_trimmed_line) {
                    break;
                } else {
                    // Try to tell the user specifically where the error is at
                    println!("You have a problem in line {} with content: {} in the configuration file located at {}", current_line_number, current_line, config_folder_path.display());
                    return;
                }
            }
        }

        current_line.clear();
    }

    // Check for errors and see if there' s a valid config queue of commands to execute
    if let ConfigParserState::SearchingForConfigBlock = parser_config_state {
        println!("The configuration name you specified in the program parameters was not found in the file. Please, check for spelling mistakes or other mistakes.");
    } else if current_user_args.generate_documentation && !found_documentation_config {
        println!("In the program parameters you specified you wanted to use a programming documentation engine. However, in the configuration file in the block of the config you specified, there is no definition of the documentation engine. Add this to the config file: Documentation([name_of_the_engine])");
    } else {
        create_dir_all(&current_user_args.project_name).unwrap();

        for command in commands_to_execute_queue.iter() {
            match command {
                ConfigFunctions::CreateFolder(folder_path) => {
                    create_dir_all(&folder_path).unwrap();
                }
                ConfigFunctions::CreateEmptyFiles(file_path) => {
                    File::create(&file_path).unwrap();
                }
                ConfigFunctions::CreateNonEmptyFile(file_path, text_to_write) => {
                    fs_handling::create_non_empty_file(&file_path, &text_to_write)
                }
            }
        }

        println!("Folder structure created with success.");
    }
}
