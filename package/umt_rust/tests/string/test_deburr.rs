use umt_rust::string::umt_deburr;

#[test]
fn test_removes_acute_accents() {
    assert_eq!(umt_deburr("déjà vu"), "deja vu");
}

#[test]
fn test_removes_multiple_diacritical_marks() {
    assert_eq!(umt_deburr("Crème brûlée"), "Creme brulee");
}

#[test]
fn test_handles_mix_of_accented_and_plain() {
    assert_eq!(umt_deburr("résumé café"), "resume cafe");
}

#[test]
fn test_leaves_plain_ascii_unchanged() {
    assert_eq!(umt_deburr("hello world"), "hello world");
}

#[test]
fn test_handles_empty_string() {
    assert_eq!(umt_deburr(""), "");
}
