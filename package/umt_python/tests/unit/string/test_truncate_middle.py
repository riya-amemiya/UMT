import unittest

from src.string import truncate_middle


class TestTruncateMiddle(unittest.TestCase):
    def test_docstring_examples(self):
        self.assertEqual(truncate_middle("Hello World", 8), "Hel...ld")
        self.assertEqual(truncate_middle("Hello World", 7, "~"), "Hel~rld")
        self.assertEqual(truncate_middle("Hello", 10), "Hello")
        self.assertEqual(truncate_middle("Hello World", 2), "..")

    def test_no_truncation_when_fits(self):
        self.assertEqual(truncate_middle("Hello World", 11), "Hello World")
        self.assertEqual(truncate_middle("", 0), "")

    def test_extra_character_goes_to_head(self):
        self.assertEqual(truncate_middle("Hello World", 10), "Hell...rld")
        self.assertEqual(truncate_middle("Hello World", 4), "H...")

    def test_never_exceeds_length(self):
        source = "abcdefghijklmnopqrstuvwxyz"
        for length in range(len(source) + 1):
            self.assertLessEqual(len(truncate_middle(source, length)), length)

    def test_separator_cut_when_length_too_small(self):
        self.assertEqual(truncate_middle("Hello World", 3), "...")
        self.assertEqual(truncate_middle("Hello World", 1), ".")
        self.assertEqual(truncate_middle("Hello World", 0), "")
        self.assertEqual(truncate_middle("Hello World", -1), "")

    def test_empty_separator(self):
        self.assertEqual(truncate_middle("abcdef", 4, ""), "abef")
        self.assertEqual(truncate_middle("abcdef", 0, ""), "")

    def test_counts_code_points(self):
        self.assertEqual(truncate_middle("a\U0001f600bc", 3, ""), "a\U0001f600c")


if __name__ == "__main__":
    unittest.main()
