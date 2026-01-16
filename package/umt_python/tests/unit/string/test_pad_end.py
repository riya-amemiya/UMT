import unittest

from src import pad_end


class TestPadEnd(unittest.TestCase):
    def test_basic_padding(self):
        self.assertEqual(pad_end("abc", 5, "0"), "abc00")
        self.assertEqual(pad_end("hello", 10, "*"), "hello*****")

    def test_pad_string_longer_than_needed(self):
        self.assertEqual(pad_end("abc", 5, "012345"), "abc01")

    def test_target_length_less_than_string_length(self):
        self.assertEqual(pad_end("hello", 3, "*"), "hello")

    def test_empty_pad_string(self):
        self.assertEqual(pad_end("hello", 8, ""), "hello")

    def test_empty_string(self):
        self.assertEqual(pad_end("", 3, "x"), "xxx")

    def test_target_length_is_zero(self):
        self.assertEqual(pad_end("abc", 0, "0"), "abc")  # String length is already > 0

    def test_docstring_examples(self):
        self.assertEqual(pad_end("abc", 5, "0"), "abc00")
        self.assertEqual(pad_end("hello", 8, "123"), "hello123")
        self.assertEqual(pad_end("world", 3, "!"), "world")
        self.assertEqual(pad_end("test", 6, ""), "test")


if __name__ == "__main__":
    unittest.main()
