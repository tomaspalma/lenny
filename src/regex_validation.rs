use regex::Regex;

fn is_config_name(candidate: &str) -> bool {
    Regex::new(r"^[?]$").unwrap().is_match(candidate)
}

fn is_documentation_specifier(candidate: &str) -> bool {
    Regex::new(r"^$").unwrap().is_match(candidate)
}
