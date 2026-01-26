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

    def test_decimal_entities(self):
        """Test decimal numeric character references."""

        self.assertEqual(unescape_html("&#65;"), "A")
        self.assertEqual(unescape_html("&#97;"), "a")
        self.assertEqual(unescape_html("&#60;"), "<")
        self.assertEqual(unescape_html("&#62;"), ">")

        self.assertEqual(unescape_html("&#72;&#101;&#108;&#108;&#111;"), "Hello")

    def test_hex_entities(self):
        """Test hexadecimal numeric character references."""

        self.assertEqual(unescape_html("&#x41;"), "A")
        self.assertEqual(unescape_html("&#x61;"), "a")
        self.assertEqual(unescape_html("&#x3C;"), "<")
        self.assertEqual(unescape_html("&#x3E;"), ">")

        self.assertEqual(unescape_html("&#x3c;"), "<")
        self.assertEqual(unescape_html("&#x3e;"), ">")

        self.assertEqual(unescape_html("&#x48;&#x69;"), "Hi")

    def test_invalid_decimal_entities(self):
        """Test invalid decimal entities that cause ValueError or OverflowError."""

        result = unescape_html("&#99999999999999999999;")
        self.assertEqual(result, "&#99999999999999999999;")

        result = unescape_html("&#;")
        self.assertEqual(result, "&#;")

    def test_invalid_hex_entities(self):
        """Test invalid hex entities that cause ValueError or OverflowError."""

        result = unescape_html("&#xFFFFFFFFFFFFFFFFFFFF;")
        self.assertEqual(result, "&#xFFFFFFFFFFFFFFFFFFFF;")

        result = unescape_html("&#x;")
        self.assertEqual(result, "&#x;")

    def test_mixed_entities(self):
        """Test mix of named, decimal and hex entities."""
        self.assertEqual(
            unescape_html("&lt;&#65;&#x42;&gt;"),
            "<AB>",
        )
        self.assertEqual(
            unescape_html("Hello &amp; &#87;orld &#x21;"),
            "Hello & World !",
        )


if __name__ == "__main__":
    unittest.main()
