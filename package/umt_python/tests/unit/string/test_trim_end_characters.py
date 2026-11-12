import unittest

from src import trim_end_characters


class TestTrimEndCharacters(unittest.TestCase):
    def test_basic_trim(self):
        self.assertEqual(trim_end_characters("hello---", "-"), "hello")
        self.assertEqual(trim_end_characters("world...", "."), "world")

    def test_multiple_chars_to_trim(self):
        self.assertEqual(trim_end_characters("hello-.-.", "-."), "hello")

    def test_no_chars_to_trim(self):
        self.assertEqual(trim_end_characters("hello", "-"), "hello")

    def test_string_is_all_trim_chars(self):
        self.assertEqual(trim_end_characters("----", "-"), "")

    def test_empty_string(self):
        self.assertEqual(trim_end_characters("", "-"), "")

    def test_empty_trim_chars(self):
        self.assertEqual(trim_end_characters("hello", ""), "hello")

    def test_trim_chars_not_at_end(self):
        self.assertEqual(trim_end_characters("---hello", "-"), "---hello")

    def test_docstring_examples(self):
        self.assertEqual(trim_end_characters("hello!!!", "!"), "hello")
        self.assertEqual(trim_end_characters("123---", "-"), "123")
        self.assertEqual(trim_end_characters("abc123", "xyz"), "abc123")


class TestTrimEndCharactersSet(unittest.TestCase):
    def test_empty_chars_keeps_whitespace(self):
        self.assertEqual(trim_end_characters("hello  ", ""), "hello  ")

    def test_chars_are_a_set_not_a_suffix(self):
        self.assertEqual(trim_end_characters("Xabcabc", "cba"), "X")

    def test_non_ascii_chars(self):
        self.assertEqual(trim_end_characters("a\u3042\u3042", "\u3042"), "a")


if __name__ == "__main__":
    unittest.main()
