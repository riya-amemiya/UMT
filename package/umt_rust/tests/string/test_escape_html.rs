use umt_rust::string::umt_escape_html;

#[test]
fn test_escape_ampersand() {
    assert_eq!(umt_escape_html("Tom & Jerry"), "Tom &amp; Jerry");
}

#[test]
fn test_escape_less_than() {
    assert_eq!(umt_escape_html("5 < 10"), "5 &lt; 10");
}

#[test]
fn test_escape_greater_than() {
    assert_eq!(umt_escape_html("10 > 5"), "10 &gt; 5");
}

#[test]
fn test_escape_double_quotes() {
    assert_eq!(umt_escape_html("Say \"Hello\""), "Say &quot;Hello&quot;");
}

#[test]
fn test_escape_single_quotes() {
    assert_eq!(umt_escape_html("It's working"), "It&#39;s working");
}

#[test]
fn test_escape_all_special_characters() {
    let input = "<script>alert(\"XSS & 'injection'\");</script>";
    let expected = "&lt;script&gt;alert(&quot;XSS &amp; &#39;injection&#39;&quot;);&lt;/script&gt;";
    assert_eq!(umt_escape_html(input), expected);
}

#[test]
fn test_handle_empty_string() {
    assert_eq!(umt_escape_html(""), "");
}

#[test]
fn test_handle_string_with_no_special_characters() {
    assert_eq!(umt_escape_html("Hello World"), "Hello World");
}

#[test]
fn test_handle_string_with_only_special_characters() {
    assert_eq!(umt_escape_html("&<>\"'"), "&amp;&lt;&gt;&quot;&#39;");
}

#[test]
fn test_handle_repeated_special_characters() {
    assert_eq!(umt_escape_html("&&&"), "&amp;&amp;&amp;");
    assert_eq!(umt_escape_html("<<<"), "&lt;&lt;&lt;");
}

#[test]
fn test_handle_mixed_content() {
    let input = "User input: <b>Hello & 'World'</b>";
    let expected = "User input: &lt;b&gt;Hello &amp; &#39;World&#39;&lt;/b&gt;";
    assert_eq!(umt_escape_html(input), expected);
}

#[test]
fn test_handle_html_attributes() {
    let input = "<img src=\"test.jpg\" alt=\"Tom & Jerry's picture\">";
    let expected =
        "&lt;img src=&quot;test.jpg&quot; alt=&quot;Tom &amp; Jerry&#39;s picture&quot;&gt;";
    assert_eq!(umt_escape_html(input), expected);
}

#[test]
fn test_handle_javascript_code() {
    let input = "if (x < 5 && y > 3) { alert(\"Hello 'World'\"); }";
    let expected =
        "if (x &lt; 5 &amp;&amp; y &gt; 3) { alert(&quot;Hello &#39;World&#39;&quot;); }";
    assert_eq!(umt_escape_html(input), expected);
}
