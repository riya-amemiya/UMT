use umt_rust::string::{umt_escape_html, umt_unescape_html};

#[test]
fn test_unescape_ampersand() {
    assert_eq!(umt_unescape_html("Tom &amp; Jerry"), "Tom & Jerry");
}

#[test]
fn test_unescape_less_than() {
    assert_eq!(umt_unescape_html("5 &lt; 10"), "5 < 10");
}

#[test]
fn test_unescape_greater_than() {
    assert_eq!(umt_unescape_html("10 &gt; 5"), "10 > 5");
}

#[test]
fn test_unescape_double_quotes() {
    assert_eq!(umt_unescape_html("Say &quot;Hello&quot;"), "Say \"Hello\"");
}

#[test]
fn test_unescape_single_quotes() {
    assert_eq!(umt_unescape_html("It&#39;s working"), "It's working");
}

#[test]
fn test_unescape_hexadecimal_single_quotes() {
    assert_eq!(umt_unescape_html("It&#x27;s working"), "It's working");
}

#[test]
fn test_unescape_all_basic_entities() {
    let input = "&lt;script&gt;alert(&quot;XSS &amp; &#39;injection&#39;&quot;);&lt;/script&gt;";
    let expected = "<script>alert(\"XSS & 'injection'\");</script>";
    assert_eq!(umt_unescape_html(input), expected);
}

#[test]
fn test_handle_empty_string() {
    assert_eq!(umt_unescape_html(""), "");
}

#[test]
fn test_handle_string_with_no_entities() {
    assert_eq!(umt_unescape_html("Hello World"), "Hello World");
}

#[test]
fn test_handle_string_with_only_entities() {
    assert_eq!(umt_unescape_html("&amp;&lt;&gt;&quot;&#39;"), "&<>\"'");
}

#[test]
fn test_handle_repeated_entities() {
    assert_eq!(umt_unescape_html("&amp;&amp;&amp;"), "&&&");
    assert_eq!(umt_unescape_html("&lt;&lt;&lt;"), "<<<");
}

#[test]
fn test_handle_mixed_content() {
    let input = "User input: &lt;b&gt;Hello &amp; &#39;World&#39;&lt;/b&gt;";
    let expected = "User input: <b>Hello & 'World'</b>";
    assert_eq!(umt_unescape_html(input), expected);
}

#[test]
fn test_handle_html_attributes() {
    let input =
        "&lt;img src=&quot;test.jpg&quot; alt=&quot;Tom &amp; Jerry&#39;s picture&quot;&gt;";
    let expected = "<img src=\"test.jpg\" alt=\"Tom & Jerry's picture\">";
    assert_eq!(umt_unescape_html(input), expected);
}

#[test]
fn test_handle_javascript_code() {
    let input = "if (x &lt; 5 &amp;&amp; y &gt; 3) { alert(&quot;Hello &#39;World&#39;&quot;); }";
    let expected = "if (x < 5 && y > 3) { alert(\"Hello 'World'\"); }";
    assert_eq!(umt_unescape_html(input), expected);
}

#[test]
fn test_handle_decimal_numeric_character_references() {
    assert_eq!(umt_unescape_html("&#65;"), "A");
    assert_eq!(umt_unescape_html("&#8364;"), "€");
    assert_eq!(umt_unescape_html("&#128512;"), "😀");
}

#[test]
fn test_handle_hexadecimal_numeric_character_references() {
    assert_eq!(umt_unescape_html("&#x41;"), "A");
    assert_eq!(umt_unescape_html("&#x20AC;"), "€");
    assert_eq!(umt_unescape_html("&#x1F600;"), "😀");
}

#[test]
fn test_handle_mixed_case_hexadecimal_references() {
    assert_eq!(umt_unescape_html("&#x41;"), "A");
    // Note: uppercase X should not match
    assert_eq!(umt_unescape_html("&#X41;"), "&#X41;");
    assert_eq!(umt_unescape_html("&#x20ac;"), "€");
    assert_eq!(umt_unescape_html("&#x20AC;"), "€");
}

#[test]
fn test_handle_extended_entities() {
    assert_eq!(umt_unescape_html("&#x2F;"), "/");
    assert_eq!(umt_unescape_html("&#x60;"), "`");
    assert_eq!(umt_unescape_html("&#x3D;"), "=");
}

#[test]
fn test_handle_invalid_numeric_references_gracefully() {
    assert_eq!(umt_unescape_html("&#;"), "&#;");
    assert_eq!(umt_unescape_html("&#x;"), "&#x;");
    assert_eq!(umt_unescape_html("&#invalid;"), "&#invalid;");
    assert_eq!(umt_unescape_html("&#xinvalid;"), "&#xinvalid;");
}

#[test]
fn test_handle_malformed_entities() {
    assert_eq!(umt_unescape_html("&lt"), "&lt");
    assert_eq!(umt_unescape_html("&unknown;"), "&unknown;");
    assert_eq!(umt_unescape_html("&#"), "&#");
    assert_eq!(umt_unescape_html("&nonexistent;"), "&nonexistent;");
    assert_eq!(umt_unescape_html("&notinmap;"), "&notinmap;");
    assert_eq!(umt_unescape_html("&test;"), "&test;");
}

#[test]
fn test_handle_complex_html_document() {
    let input = "&lt;!DOCTYPE html&gt;
&lt;html&gt;
&lt;head&gt;
    &lt;title&gt;Test &amp; Demo&lt;/title&gt;
&lt;/head&gt;
&lt;body&gt;
    &lt;p&gt;Hello &#39;World&#39; &amp; welcome!&lt;/p&gt;
&lt;/body&gt;
&lt;/html&gt;";

    let expected = "<!DOCTYPE html>
<html>
<head>
    <title>Test & Demo</title>
</head>
<body>
    <p>Hello 'World' & welcome!</p>
</body>
</html>";

    assert_eq!(umt_unescape_html(input), expected);
}

#[test]
fn test_be_inverse_of_escape_html() {
    let test_cases = vec![
        "Hello & World",
        "<script>alert(\"test\");</script>",
        "It's a 'test' & more",
        "5 < 10 > 3",
        "",
        "No special chars",
    ];

    for test_case in test_cases {
        let escaped = umt_escape_html(test_case);
        let unescaped = umt_unescape_html(&escaped);
        assert_eq!(unescaped, test_case);
    }
}

#[test]
fn test_handle_unicode_characters_in_numeric_references() {
    assert_eq!(umt_unescape_html("&#12354;"), "あ");
    assert_eq!(umt_unescape_html("&#x3042;"), "あ");
    assert_eq!(umt_unescape_html("&#8226;"), "•");
    assert_eq!(umt_unescape_html("&#x2022;"), "•");
}

#[test]
fn test_preserve_already_unescaped_content() {
    let input = "Already < unescaped & content with 'quotes'";
    assert_eq!(umt_unescape_html(input), input);
}
