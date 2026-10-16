import unittest

from src.string import escape_html


class TestEscapeHtml(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(
            escape_html("<script>alert('XSS')</script>"),
            "&lt;script&gt;alert(&#39;XSS&#39;)&lt;/script&gt;",
        )
        self.assertEqual(escape_html("Tom & Jerry"), "Tom &amp; Jerry")
        self.assertEqual(escape_html('"Hello"'), "&quot;Hello&quot;")

    def test_edge_cases(self):
        self.assertEqual(escape_html(""), "")
        self.assertEqual(escape_html("hello"), "hello")

    def test_docstring_example(self):
        self.assertEqual(
            escape_html("<script>alert('XSS')</script>"),
            "&lt;script&gt;alert(&#39;XSS&#39;)&lt;/script&gt;",
        )
        self.assertEqual(escape_html("Tom & Jerry"), "Tom &amp; Jerry")
        self.assertEqual(escape_html('"Hello"'), "&quot;Hello&quot;")

    def test_individual_specials(self):
        self.assertEqual(escape_html("5 < 10"), "5 &lt; 10")
        self.assertEqual(escape_html("10 > 5"), "10 &gt; 5")
        self.assertEqual(escape_html("It's working"), "It&#39;s working")
        self.assertEqual(escape_html('Say "Hello"'), "Say &quot;Hello&quot;")

    def test_all_specials_together(self):
        self.assertEqual(escape_html("&<>\"'"), "&amp;&lt;&gt;&quot;&#39;")
        self.assertEqual(
            escape_html("<script>alert(\"XSS & 'injection'\");</script>"),
            "&lt;script&gt;alert(&quot;XSS &amp; &#39;injection&#39;&quot;);&lt;/script&gt;",
        )

    def test_repeated_and_mixed(self):
        self.assertEqual(escape_html("&&&"), "&amp;&amp;&amp;")
        self.assertEqual(escape_html("<<<"), "&lt;&lt;&lt;")
        self.assertEqual(
            escape_html("User input: <b>Hello & 'World'</b>"),
            "User input: &lt;b&gt;Hello &amp; &#39;World&#39;&lt;/b&gt;",
        )
        self.assertEqual(
            escape_html('<img src="test.jpg" alt="Tom & Jerry\'s picture">'),
            "&lt;img src=&quot;test.jpg&quot; alt=&quot;Tom &amp; Jerry&#39;s picture&quot;&gt;",
        )
        self.assertEqual(
            escape_html("if (x < 5 && y > 3) { alert(\"Hello 'World'\"); }"),
            "if (x &lt; 5 &amp;&amp; y &gt; 3) { alert(&quot;Hello &#39;World&#39;&quot;); }",
        )

    def test_slash_backtick_cjk_unchanged(self):
        self.assertEqual(escape_html("</script>"), "&lt;/script&gt;")
        self.assertEqual(
            escape_html("Use `template` literals"), "Use `template` literals"
        )
        self.assertEqual(escape_html("こんにちは World"), "こんにちは World")
        self.assertEqual(escape_html("a & こんにちは < b"), "a &amp; こんにちは &lt; b")


if __name__ == "__main__":
    unittest.main()
