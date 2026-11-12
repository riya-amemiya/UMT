import unittest

from src import trim_start_characters


class TestTrimStartCharacters(unittest.TestCase):
    def test_basic_trim(self):
        self.assertEqual(trim_start_characters("---hello", "-"), "hello")
        self.assertEqual(trim_start_characters("...world", "."), "world")

    def test_multiple_chars_to_trim(self):
        self.assertEqual(trim_start_characters("-.-.hello", "-."), "hello")

    def test_no_chars_to_trim(self):
        self.assertEqual(trim_start_characters("hello", "-"), "hello")

    def test_string_is_all_trim_chars(self):
        self.assertEqual(trim_start_characters("----", "-"), "")

    def test_empty_string(self):
        self.assertEqual(trim_start_characters("", "-"), "")

    def test_empty_trim_chars(self):
        self.assertEqual(trim_start_characters("hello", ""), "hello")

    def test_trim_chars_not_at_start(self):
        self.assertEqual(trim_start_characters("hello---", "-"), "hello---")

    def test_docstring_examples(self):
        self.assertEqual(trim_start_characters("!!!hello", "!"), "hello")
        self.assertEqual(trim_start_characters("---123", "-"), "123")
        self.assertEqual(trim_start_characters("abc123", "xyz"), "abc123")


class TestTrimStartCharactersSet(unittest.TestCase):
    def test_empty_chars_keeps_whitespace(self):
        self.assertEqual(trim_start_characters("  hello", ""), "  hello")

    def test_chars_are_a_set_not_a_prefix(self):
        self.assertEqual(trim_start_characters("abcabcX", "cba"), "X")

    def test_non_ascii_chars(self):
        self.assertEqual(trim_start_characters("\u3042\u3042a", "\u3042"), "a")


if __name__ == "__main__":
    unittest.main()
