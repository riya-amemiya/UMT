import re
import unittest

from src.string import words


class TestWords(unittest.TestCase):
    def test_splits_camel_case(self):
        self.assertEqual(words("helloWorld"), ["hello", "World"])

    def test_splits_acronym_followed_by_word(self):
        self.assertEqual(words("XMLHttpRequest"), ["XML", "Http", "Request"])

    def test_splits_on_dashes_and_underscores(self):
        self.assertEqual(words("foo-bar_baz"), ["foo", "bar", "baz"])

    def test_splits_on_whitespace(self):
        self.assertEqual(words("hello world"), ["hello", "world"])

    def test_empty_string(self):
        self.assertEqual(words(""), [])

    def test_custom_pattern(self):
        self.assertEqual(words("a1 b2 c3", re.compile(r"[a-z]\d")), ["a1", "b2", "c3"])

    def test_custom_pattern_no_match(self):
        self.assertEqual(words("hello", re.compile(r"\d+")), [])


if __name__ == "__main__":
    unittest.main()
