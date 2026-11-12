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

#[test]
fn test_removes_spacing_diacritics() {
    assert_eq!(umt_deburr("a^b`c"), "abc");
    assert_eq!(umt_deburr("x\u{a8}y\u{b4}z"), "xyz");
}

#[test]
fn test_removes_kana_sound_marks() {
    assert_eq!(
        umt_deburr("\u{30e9}\u{30fc}\u{30e1}\u{30f3}"),
        "\u{30e9}\u{30e1}\u{30f3}"
    );
    assert_eq!(umt_deburr("\u{304c}\u{304e}"), "\u{304b}\u{304d}");
}

#[test]
fn test_keeps_combining_marks_without_diacritic_property() {
    assert_eq!(umt_deburr("\u{e01}\u{e38}"), "\u{e01}\u{e38}");
    assert_eq!(umt_deburr("e\u{20d0}"), "e\u{20d0}");
}
