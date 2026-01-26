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
fn test_plural_formatter_empty_args() {
    let formatters = create_default_formatters();
    let plural = formatters.get("plural").unwrap();
    // With no args, should return empty strings
    assert_eq!(plural("1", &[]), "");
    assert_eq!(plural("5", &[]), "");
}

#[test]
fn test_plural_formatter_invalid_number() {
    let formatters = create_default_formatters();
    let plural = formatters.get("plural").unwrap();
    // Invalid number should default to 0, returning plural form
    assert_eq!(
        plural("abc", &["item".to_string(), "items".to_string()]),
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

#[test]
fn test_pad_formatter_no_padding_needed() {
    let formatters = create_default_formatters();
    let pad = formatters.get("pad").unwrap();
    // Value is already long enough
    assert_eq!(pad("12345", &["3".to_string(), "0".to_string()]), "12345");
    assert_eq!(pad("abc", &["2".to_string()]), "abc");
}

#[test]
fn test_pad_formatter_empty_pad_char() {
    let formatters = create_default_formatters();
    let pad = formatters.get("pad").unwrap();
    // Empty pad char string should default to '0'
    assert_eq!(pad("5", &["3".to_string(), "".to_string()]), "005");
}

#[test]
fn test_number_formatter() {
    let formatters = create_default_formatters();
    let number = formatters.get("number").unwrap();

    // Basic integer formatting
    assert_eq!(number("42", &[]), "42");
    // Float formatting with max_fraction specified to avoid precision issues
    assert_eq!(
        number(
            "3.14",
            &["en".to_string(), "0".to_string(), "2".to_string()]
        ),
        "3.14"
    );
}

#[test]
fn test_number_formatter_with_min_fraction() {
    let formatters = create_default_formatters();
    let number = formatters.get("number").unwrap();

    // With minimum fraction digits - use max_fraction to control precision
    assert_eq!(
        number("42", &["en".to_string(), "2".to_string(), "2".to_string()]),
        "42.00"
    );
    assert_eq!(
        number("3.1", &["en".to_string(), "3".to_string(), "3".to_string()]),
        "3.100"
    );
}

#[test]
fn test_number_formatter_with_max_fraction() {
    let formatters = create_default_formatters();
    let number = formatters.get("number").unwrap();

    // With max fraction digits
    assert_eq!(
        number(
            "3.14159",
            &["en".to_string(), "0".to_string(), "2".to_string()]
        ),
        "3.14"
    );
}

#[test]
fn test_number_formatter_invalid_input() {
    let formatters = create_default_formatters();
    let number = formatters.get("number").unwrap();

    // Invalid number should default to 0
    assert_eq!(number("abc", &[]), "0");
}

#[test]
fn test_number_formatter_no_decimal() {
    let formatters = create_default_formatters();
    let number = formatters.get("number").unwrap();

    // Integer with min fraction = 0 should not have decimal point
    assert_eq!(
        number("42", &["en".to_string(), "0".to_string(), "2".to_string()]),
        "42"
    );
}
