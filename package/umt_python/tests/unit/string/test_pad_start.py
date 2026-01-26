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

    def test_empty_pad_string_raises_error(self):
        with self.assertRaises(ValueError):
            pad_start("hello", 8, "")

    def test_empty_string(self):
        self.assertEqual(pad_start("", 3, "x"), "xxx")

    def test_target_length_is_zero(self):
        self.assertEqual(pad_start("abc", 0, "0"), "abc")

    def test_docstring_examples(self):
        self.assertEqual(pad_start("123", 5, "0"), "00123")
        self.assertEqual(pad_start("abc", 8, "def"), "defdeabc")
        self.assertEqual(pad_start("world", 3, "!"), "world")


if __name__ == "__main__":
    unittest.main()
