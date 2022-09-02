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

[exam]
CreateFolders(group1, group2, group3, group4)

```

