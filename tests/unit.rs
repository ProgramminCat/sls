use sls::*;

#[test]
fn test_parse_size() {
    assert_eq!(parse_size("10KB"), Some(10 * 1024));
}