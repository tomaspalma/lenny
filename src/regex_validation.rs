use regex::Regex;
use lazy_static::lazy_static;

fn is_config_name(candidate: &str) -> bool {
    lazy_static! {
         static ref CURRENT_REGEX: Regex = Regex::new(r"^[?]$").unwrap();
    }

    CURRENT_REGEX.is_match(candidate)
}

fn is_documentation_specifier(candidate: &str) -> bool {
    lazy_static! {
       static ref CURRENT_REGEX: Regex = Regex::new(r"^$").unwrap();
    }
    
    CURRENT_REGEX.is_match(candidate)
}
