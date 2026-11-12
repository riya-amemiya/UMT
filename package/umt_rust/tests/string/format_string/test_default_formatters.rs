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
    // Empty pad char string adds no padding, like padStart with ""
    assert_eq!(pad("5", &["3".to_string(), "".to_string()]), "5");
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

    // Invalid number is formatted as NaN
    assert_eq!(number("abc", &[]), "NaN");
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

#[test]
fn test_currency_formatter() {
    let formatters = create_default_formatters();
    let currency = formatters.get("currency").unwrap();
    assert_eq!(currency("1234.5", &[]), "$1,234.50");
    assert_eq!(
        currency("1234.5", &["en-US".to_string(), "EUR".to_string()]),
        "\u{20ac}1,234.50"
    );
    assert_eq!(currency(" 2 ", &["en-US".to_string()]), "$2.00");
}

#[test]
fn test_number_formatter_groups_digits() {
    let formatters = create_default_formatters();
    let number = formatters.get("number").unwrap();
    assert_eq!(number("1234.5", &[]), "1,234.5");
    assert_eq!(number(" 1234.5 ", &[]), "1,234.5");
    assert_eq!(
        number(
            "1234.5",
            &["en-US".to_string(), "2".to_string(), "2".to_string()]
        ),
        "1,234.50"
    );
    assert_eq!(number("1234.5", &["de-DE".to_string()]), "1.234,5");
    assert_eq!(
        number(
            "1.5",
            &["en-US".to_string(), "x".to_string(), "y".to_string()]
        ),
        "1.5"
    );
    assert_eq!(number("abc", &[]), "NaN");
}

#[test]
fn test_pad_formatter_length_follows_javascript_number_rules() {
    let formatters = create_default_formatters();
    let pad = formatters.get("pad").unwrap();
    assert_eq!(pad("5", &[]), "05");
    assert_eq!(pad("5", &["abc".to_string()]), "5");
    assert_eq!(pad("5", &["-1".to_string()]), "5");
    assert_eq!(pad("5", &["Infinity".to_string()]), "5");
    assert_eq!(pad("5", &["-Infinity".to_string()]), "5");
    assert_eq!(pad("5", &["".to_string()]), "5");
    assert_eq!(pad("5", &["0x4".to_string()]), "0005");
    assert_eq!(pad("5", &["0o4".to_string()]), "0005");
    assert_eq!(pad("5", &["0B100".to_string()]), "0005");
    assert_eq!(pad("5", &["0x+4".to_string()]), "5");
    assert_eq!(pad("5", &["0xZ".to_string()]), "5");
    assert_eq!(
        pad("5", &["1e1".to_string(), "x".to_string()]),
        "xxxxxxxxx5"
    );
    assert_eq!(pad("5", &["1e".to_string()]), "5");
    assert_eq!(pad("5", &["--".to_string()]), "5");
}

#[test]
fn test_pad_formatter_uses_whole_pad_string() {
    let formatters = create_default_formatters();
    let pad = formatters.get("pad").unwrap();
    assert_eq!(pad("5", &["4".to_string(), "ab".to_string()]), "aba5");
    assert_eq!(pad("5", &["3".to_string(), " ".to_string()]), "  5");
    assert_eq!(pad("\u{3042}", &["5".to_string()]), "0000\u{3042}");
}

#[test]
fn test_plural_formatter_follows_javascript_number_rules() {
    let formatters = create_default_formatters();
    let plural = formatters.get("plural").unwrap();
    let args = ["item".to_string(), "items".to_string()];
    assert_eq!(plural(" 1 ", &args), "item");
    assert_eq!(plural("1.0", &args), "item");
    assert_eq!(plural("abc", &args), "items");
    assert_eq!(plural("", &args), "items");
    assert_eq!(plural("1", &[]), "");
    assert_eq!(plural("2", &[]), "");
}
