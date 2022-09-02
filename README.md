# Lenny 

## What is it?

Lenny is a folder structure automatic generation tool that reads from an easily customized text config file

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
CreateEmptyFiles(todo.txt)
CreateNonEmptyFile(src/main.cpp, 
#include <iostream>

int main(void) {
	std::cout << "Hello world";
})

```

## How to execute the program
```
USAGE:
    lenny [OPTIONS] --config <CONFIG_NAME> --name <PROJECT_NAME>

OPTIONS:
    -a, --alternativecfg               Activate search in [config_name].txt instead of config.txt
    -c, --config <CONFIG_NAME>         Name of the config title in the config file
    -d, --docs                         Generate documentation configuration files
    -g, --git <GIT_REPOSITORY_LINK>    Links with already created repository
    -h, --help                         Print help information
    -n, --name <PROJECT_NAME>          Name of the main folder of the project
```
