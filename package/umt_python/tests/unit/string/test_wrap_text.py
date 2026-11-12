import unittest

from src.string import wrap_text


class TestWrapText(unittest.TestCase):
    def test_docstring_examples(self):
        self.assertEqual(wrap_text("The quick brown fox", 10), "The quick\nbrown fox")
        self.assertEqual(wrap_text("a extraordinary b", 5), "a\nextraordinary\nb")

    def test_line_exactly_width(self):
        self.assertEqual(wrap_text("ab cd", 5), "ab cd")
        self.assertEqual(wrap_text("ab cd", 4), "ab\ncd")

    def test_collapses_separator_runs(self):
        self.assertEqual(wrap_text("  a \t b\f\vc  ", 10), "a b c")

    def test_keeps_line_breaks(self):
        self.assertEqual(wrap_text("aaa bbb\nccc", 3), "aaa\nbbb\nccc")
        self.assertEqual(wrap_text("a\n\nb", 5), "a\n\nb")
        self.assertEqual(wrap_text("ab\r\ncd", 10), "ab\ncd")

    def test_zero_width(self):
        self.assertEqual(wrap_text("a b", 0), "a\nb")

    def test_empty_or_blank_input(self):
        self.assertEqual(wrap_text("", 10), "")
        self.assertEqual(wrap_text("   ", 10), "")

    def test_measures_code_points(self):
        self.assertEqual(wrap_text("\U0001f600 a", 3), "\U0001f600 a")


if __name__ == "__main__":
    unittest.main()
