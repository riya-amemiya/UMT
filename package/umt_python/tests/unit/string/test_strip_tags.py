import unittest

from src.string import strip_tags


class TestStripTags(unittest.TestCase):
    def test_removes_simple_tags(self):
        self.assertEqual(strip_tags("<p>Hello</p>"), "Hello")

    def test_removes_nested_tags_keep_text(self):
        self.assertEqual(strip_tags("<p>Hello <b>World</b></p>"), "Hello World")

    def test_removes_tags_with_attributes(self):
        self.assertEqual(strip_tags('<a href="https://example.com">link</a>'), "link")

    def test_removes_self_closing_tags(self):
        self.assertEqual(strip_tags("line1<br/>line2"), "line1line2")

    def test_removes_nested_injection(self):
        self.assertEqual(strip_tags("<sc<script>ript>"), "")

    def test_leaves_text_without_tags(self):
        self.assertEqual(strip_tags("plain text"), "plain text")

    def test_empty_string(self):
        self.assertEqual(strip_tags(""), "")


if __name__ == "__main__":
    unittest.main()
