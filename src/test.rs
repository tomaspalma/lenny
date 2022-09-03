#[cfg(tests)]
use super::*;

#[test]
fn test_regex_config_line() {
    assert_eq!(regex_validation::is_config_name("[cpp]"), true);
}
