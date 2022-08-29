use regex::Regex;
use lazy_static::lazy_static;

pub fn is_config_name(candidate: &str) -> bool {
    lazy_static! {
         static ref CURRENT_REGEX: Regex = Regex::new(r"^\s*\[[a-zA-Z]{1,}\]\s*$").unwrap();
    }

    CURRENT_REGEX.is_match(candidate)
}

pub fn is_documentation_specifier(candidate: &str) -> bool {
    lazy_static! {
       static ref CURRENT_REGEX: Regex = Regex::new(r"^$").unwrap();
    }
    
    CURRENT_REGEX.is_match(candidate)
}
