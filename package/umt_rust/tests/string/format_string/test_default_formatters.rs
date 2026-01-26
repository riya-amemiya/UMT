use umt_rust::string::format_string::*;

#[test]
fn test_upper_formatter() {
    let formatters = create_default_formatters();
    let upper = formatters.get("upper").unwrap();
    assert_eq!(upper("hello", &[]), "HELLO");
    assert_eq!(upper("World", &[]), "WORLD");
}

#[test]
fn test_lower_formatter() {
    let formatters = create_default_formatters();
    let lower = formatters.get("lower").unwrap();
    assert_eq!(lower("HELLO", &[]), "hello");
    assert_eq!(lower("World", &[]), "world");
}

#[test]
fn test_plural_formatter() {
    let formatters = create_default_formatters();
    let plural = formatters.get("plural").unwrap();
    assert_eq!(
        plural("1", &["item".to_string(), "items".to_string()]),
        "item"
    );
    assert_eq!(
        plural("5", &["item".to_string(), "items".to_string()]),
        "items"
    );
    assert_eq!(
        plural("0", &["item".to_string(), "items".to_string()]),
        "items"
    );
}

#[test]
fn test_pad_formatter() {
    let formatters = create_default_formatters();
    let pad = formatters.get("pad").unwrap();
    assert_eq!(pad("42", &["4".to_string(), "0".to_string()]), "0042");
    assert_eq!(pad("5", &["3".to_string(), "0".to_string()]), "005");
    assert_eq!(pad("hi", &["5".to_string(), "#".to_string()]), "###hi");
}

#[test]
fn test_pad_formatter_defaults() {
    let formatters = create_default_formatters();
    let pad = formatters.get("pad").unwrap();
    assert_eq!(pad("5", &[]), "05");
}
