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


if __name__ == "__main__":
    unittest.main()
