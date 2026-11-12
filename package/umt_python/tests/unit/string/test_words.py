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


class TestWordsBoundaries(unittest.TestCase):
    def test_number_followed_by_uppercase(self):
        self.assertEqual(words("a1B"), ["a1", "B"])
        self.assertEqual(words("1A"), ["1", "A"])
        self.assertEqual(words("x\u00b2Y"), ["x\u00b2", "Y"])

    def test_uppercase_followed_by_number(self):
        self.assertEqual(words("A1"), ["A1"])
        self.assertEqual(words("HTML5Parser"), ["HTML5", "Parser"])

    def test_uppercase_run(self):
        self.assertEqual(words("AB"), ["AB"])
        self.assertEqual(words("ABc"), ["A", "Bc"])
        self.assertEqual(words("ABCdef"), ["AB", "Cdef"])
        self.assertEqual(words("aBC"), ["a", "BC"])

    def test_non_ascii_letters(self):
        self.assertEqual(words("\u00e9\u00c9"), ["\u00e9", "\u00c9"])
        self.assertEqual(words("\u00c0\u00c9\u00ee"), ["\u00c0", "\u00c9\u00ee"])
        self.assertEqual(words("\u65e5\u672c\u8a9eText"), ["\u65e5\u672c\u8a9eText"])

    def test_titlecase_letter_is_not_a_boundary(self):
        self.assertEqual(words("\u01c5a"), ["\u01c5a"])

    def test_separator_runs(self):
        self.assertEqual(words("foo  bar"), ["foo", "bar"])
        self.assertEqual(words(" "), [])
        self.assertEqual(words("--foo--"), ["foo"])


if __name__ == "__main__":
    unittest.main()
