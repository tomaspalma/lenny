# Lenny 
**Some notes**

This is a begginer project and feedback is appreciated.

Still only works on Linux and probably mac os. Other oses in the future.

## What is it?

Lenny is a folder structure automatic generation tool that reads from an easily customized text config file

## Executing and using the program

**How to execute it on linux**
1) Clone the repository -> ```git clone https:://github.com/tomaspalma/lenny``` 
2) Create a folder and file inside ~/.config -> ```mkdir ~/.config/lenny && touch ~/.config/lenny/config.txt```. You can also use the config.txt in the repository as a template. 
3) Go the folder where you cloned the repository and compile the program -> ```cargo build --release```
4) (OPTIONAL) If you want you can copy the executable inside ```target/release/``` to ```/usr/bin```

**Program options**
```
USAGE:
    lenny [OPTIONS] --config <CONFIG_NAME> --name <PROJECT_NAME>

OPTIONS:
    -a, --alternativecfg               (OPTIONAL) Activate search in [config_name].txt instead of
                                       config.txt
    -c, --config <CONFIG_NAME>         Name of the config title in the config file
    -d, --docs                         (OPTIONAL) Generate documentation configuration files
    -g, --git <GIT_REPOSITORY_LINK>    (OPTIONAL) Links with already created repository
    -h, --help                         Print help information
    -n, --name <PROJECT_NAME>          Name of the main folder of the project
```

**Examples of using those options**

- Example 1: I want to create a folder in my home directory called lenny that uses the configuration with the name cpp that is defined in the general config file ```config.txt```:

```lenny -n lenny -c cpp```

- Example 2: I want to use a separate config file for cpp and I created a ```cpp.txt``` file and wrote the "functions" I want to execute:

```lenny -n lenny -c cpp -a```

**Functions available to use in the configurations**:

**CreateFolders(folder_path_1, folder_path_2, ....)**

Creates all the folders passed in the parameter under a parent folder which will have the name of the ```project_name``` passed in the parameters while calling the program. 

For example, if I execute at my home directory ```~``` with the ```project_name``` lenny and want to create a path ```include/headers``` it would create ```~/project_name/include/headers```

**CreateEmptyFiles(file_path_1, file_path_2, ...)**

Creates the files passed in the arguments. 

Remember that if you want to create for example a file ```main.cpp``` in a folder named ```src``` if you write in the config file ```CreateEmptyFiles(src/main.cpp)``` and before you didn't create a folder named ```src``` it won't work and will result in an error.

**CreateNonEmptyFile(file_path, file_content_to_write)**

Creates a file with some text of your choice.

Examples of how to aplly these functions can be seen below.

## Configuration Files

There are two ways to organize specific folder generator configurations:
- Put them into a general configuration file that is created upon first utilization of this tool
- Create a specific configuration file named [configuration_name].txt

And what exactly are these configurations. Here' s an example config.txt with more than one configuration:
```
[cpp]
CreateFolders(include, src, docs, tests, public)
CreateEmptyFiles(todo.txt)
CreateNonEmptyFile(src/main.cpp, 
#include <iostream>

int main(void) {
	std::cout << "Hello world";
})

[exams]
CreateFolders(group1, group2, group3, group4)

```

The names between [] are the folder structure configuration name.

If you'd prefer to have those in separate files you could create two files:
- cpp.txt
- exams.txt

And then copy the functions below the [] of each folder structure configuration to those files. For example, we could justcrate a configuration file called cpp.txt and put this inside of it:

```
CreateFolders(include, src, docs, tests, public)
// This is a comment
CreateEmptyFiles(todo.txt)
# This is also a comment
CreateNonEmptyFile(src/main.cpp, 
#include <iostream>

int main(void) {
	std::cout << "Hello world";
})

```

## Current issues and features yet to add
- A command ```lenny check``` so you can get feedback if there are problems with the configuration file without the need of running the program. This will require the tool to become more modular probably.
- Better error handling instead of just saying that there's an error in a line. The goal is to specify exactly what the problem is. Probably this will require a change in the parsing mechanism.
- The program will still panic if the user tries to create to a file whose parent folder was not created and a folder of the incomplete project will be created.
- Still need to implement configuring initilization with git and also, documentation tools
