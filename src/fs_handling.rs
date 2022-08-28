use std::fs::File; 

pub fn config_file_opener(file: &str) -> (File, bool) {
    let mut created_config_file = false;
    
    let file_handler = File::open(file).unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            let file_creation_handler = File::create(file).unwrap_or_else(|error| {
               if error.kind() == std::io::ErrorKind::PermissionDenied {
                    panic!("Insufficient permissions to create file");
               } else {
                    panic!("Error creating file: { }", error);
               }
            });
            created_config_file = true;

            return file_creation_handler;
        } else {
            panic!("Error opening file: { }", error);
        }
    });

    (file_handler, created_config_file)
}
