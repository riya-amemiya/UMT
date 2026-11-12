import unittest

from src import pad_start


class TestPadStart(unittest.TestCase):
    def test_basic_padding(self):
        self.assertEqual(pad_start("123", 5, "0"), "00123")
        self.assertEqual(pad_start("abc", 8, "def"), "defdeabc")

    def test_pad_string_longer_than_needed(self):
        self.assertEqual(pad_start("123", 5, "012345"), "01123")

    def test_target_length_less_than_string_length(self):
        self.assertEqual(pad_start("hello", 3, "*"), "hello")

    def test_empty_pad_string_returns_original_string(self):
        self.assertEqual(pad_start("hello", 8, ""), "hello")

    def test_empty_string(self):
        self.assertEqual(pad_start("", 3, "x"), "xxx")

    def test_target_length_is_zero(self):
        self.assertEqual(pad_start("abc", 0, "0"), "abc")

    def test_docstring_examples(self):
        self.assertEqual(pad_start("123", 5, "0"), "00123")
        self.assertEqual(pad_start("abc", 8, "def"), "defdeabc")
        self.assertEqual(pad_start("world", 3, "!"), "world")


class TestPadStartTypeScriptParity(unittest.TestCase):
    def test_empty_pad_string_returns_original(self):
        self.assertEqual(pad_start("abc", 5, ""), "abc")

    def test_string_already_long_enough(self):
        self.assertEqual(pad_start("abc", 2, "x"), "abc")
        self.assertEqual(pad_start("abc", 3, "x"), "abc")

    def test_multi_character_pad_is_cut_to_length(self):
        self.assertEqual(pad_start("a", 5, "\u3042\u3044"), "\u3042\u3044\u3042\u3044a")
        self.assertEqual(pad_start("a", 4, "xyz"), "xyza")

    def test_non_ascii_string(self):
        self.assertEqual(pad_start("\u3042", 3, "x"), "xx\u3042")


if __name__ == "__main__":
    unittest.main()
