use regex::Regex;
use lazy_static::lazy_static;

pub fn is_config_name(candidate: &str) -> bool {
    lazy_static! {
         static ref CURRENT_REGEX: Regex = Regex::new(r"^\s*\[[a-zA-Z]{1,}\]\s*$").unwrap();
    }

    CURRENT_REGEX.is_match(candidate)
}

pub fn is_comment(candidate: &str) -> bool {
     lazy_static! {
          static ref CURRENT_REGEX: Regex = Regex::new(r"^(\#.*)|(//.*)$").unwrap();
     }

     CURRENT_REGEX.is_match(candidate)
}

pub fn is_documentation_specifier(candidate: &str) -> bool {
    lazy_static! {
       static ref CURRENT_REGEX: Regex = Regex::new(r"^\s*Documentation([a-zA-Z]{1,})$").unwrap();
    }
    
    CURRENT_REGEX.is_match(candidate)
}

pub fn is_create_folder_line(candidate: &str) -> bool {
     lazy_static! {
          static ref CURRENT_REGEX: Regex = Regex::new(r"^\s*CreateFolders\(([a-zA-Z_\-0-9]{1,})(/[a-zA-Z_\-0-9]*)*(,\s*[a-zA-Z_\-0-9]{1,}(/[a-zA-Z_\-0-9]{1,})*)*\)\s*$").unwrap();
     }

     CURRENT_REGEX.is_match(candidate)
}

pub fn is_create_file_line(candidate: &str) -> bool { 
     lazy_static! {
          static ref CURRENT_REGEX: Regex = Regex::new(r"^\s*CreateFiles\(([a-zA-Z]{1,}/)*([a-zA-Z]{1,}.[a-z]{1,})\)\s*$").unwrap();
     }

     CURRENT_REGEX.is_match(candidate)
}
