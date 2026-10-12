import unittest

from src.string import normalize_whitespace


class TestNormalizeWhitespace(unittest.TestCase):
    def test_collapses_consecutive_spaces(self):
        self.assertEqual(normalize_whitespace("hello   world"), "hello world")

    def test_collapses_mixed_whitespace(self):
        self.assertEqual(normalize_whitespace("hello \t\n world"), "hello world")

    def test_trims_leading_and_trailing(self):
        self.assertEqual(normalize_whitespace("  hello world  "), "hello world")

    def test_only_whitespace(self):
        self.assertEqual(normalize_whitespace("  \t\n "), "")

    def test_single_spaced_unchanged(self):
        self.assertEqual(normalize_whitespace("a b c"), "a b c")

    def test_empty_string(self):
        self.assertEqual(normalize_whitespace(""), "")

    def test_docstring_example(self):
        self.assertEqual(
            normalize_whitespace("  hello   world \t\n foo "),
            "hello world foo",
        )

    def test_collapses_carriage_return_whitespace(self):
        self.assertEqual(normalize_whitespace("hello \r\n world"), "hello world")

    def test_collapses_consecutive_newlines(self):
        self.assertEqual(normalize_whitespace("a\n\n\nb"), "a b")

    def test_keeps_text_around_whitespace_runs(self):
        self.assertEqual(
            normalize_whitespace("keep \t  this \n text"),
            "keep this text",
        )


if __name__ == "__main__":
    unittest.main()
