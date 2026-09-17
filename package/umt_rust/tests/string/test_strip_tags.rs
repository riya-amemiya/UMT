use umt_rust::string::umt_strip_tags;

#[test]
fn test_removes_simple_tags() {
    assert_eq!(umt_strip_tags("<p>Hello</p>"), "Hello");
}

#[test]
fn test_removes_nested_tags_but_keeps_text() {
    assert_eq!(umt_strip_tags("<p>Hello <b>World</b></p>"), "Hello World");
}

#[test]
fn test_removes_tags_with_attributes() {
    assert_eq!(
        umt_strip_tags("<a href=\"https://example.com\">link</a>"),
        "link"
    );
}

#[test]
fn test_removes_self_closing_tags() {
    assert_eq!(umt_strip_tags("line1<br/>line2"), "line1line2");
}

#[test]
fn test_removes_nested_tag_injections() {
    assert_eq!(umt_strip_tags("<sc<script>ript>"), "");
}

#[test]
fn test_leaves_text_without_tags_unchanged() {
    assert_eq!(umt_strip_tags("plain text"), "plain text");
}

#[test]
fn test_handles_empty_string() {
    assert_eq!(umt_strip_tags(""), "");
}

#[test]
fn test_keeps_text_around_tags() {
    assert_eq!(umt_strip_tags("Hello <b>World</b>!"), "Hello World!");
}

#[test]
fn test_removes_multi_level_nested_injections() {
    assert_eq!(umt_strip_tags("<sc<sc<script>ript>ript>"), "");
}

#[test]
fn test_leaves_unclosed_angle_brackets() {
    assert_eq!(umt_strip_tags("a < b"), "a < b");
}

#[test]
fn test_removes_adjacent_tags() {
    assert_eq!(umt_strip_tags("<p></p>text<div>x</div>"), "textx");
}
