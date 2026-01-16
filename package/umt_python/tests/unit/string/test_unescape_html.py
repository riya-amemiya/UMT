import unittest

from src.string import unescape_html


class TestUnescapeHtml(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(
            unescape_html("&lt;script&gt;alert(&quot;Hello&quot;);&lt;/script&gt;"),
            '<script>alert("Hello");</script>',
        )
        self.assertEqual(unescape_html("Tom &amp; Jerry"), "Tom & Jerry")
        self.assertEqual(
            unescape_html("5 &lt; 10 &amp;&amp; 10 &gt; 5"), "5 < 10 && 10 > 5"
        )

    def test_edge_cases(self):
        self.assertEqual(unescape_html(""), "")
        self.assertEqual(unescape_html("hello"), "hello")

    def test_docstring_example(self):
        self.assertEqual(
            unescape_html("&lt;script&gt;alert(&quot;Hello&quot;);&lt;/script&gt;"),
            '<script>alert("Hello");</script>',
        )
        self.assertEqual(unescape_html("Tom &amp; Jerry"), "Tom & Jerry")


if __name__ == "__main__":
    unittest.main()
