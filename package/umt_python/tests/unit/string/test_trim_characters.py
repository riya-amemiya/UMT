import unittest

from src import trim_characters


class TestTrimCharacters(unittest.TestCase):
    def test_trim_from_both_ends(self):
        self.assertEqual(trim_characters("-.-hello-.-", "-."), "hello")
        self.assertEqual(trim_characters("...world...", "."), "world")

    def test_trim_from_start_only(self):
        self.assertEqual(trim_characters("---hello", "-"), "hello")

    def test_trim_from_end_only(self):
        self.assertEqual(trim_characters("hello---", "-"), "hello")

    def test_no_chars_to_trim(self):
        self.assertEqual(trim_characters("hello", "-"), "hello")

    def test_string_is_all_trim_chars(self):
        self.assertEqual(trim_characters("-----", "-"), "")

    def test_empty_string(self):
        self.assertEqual(trim_characters("", "-"), "")

    def test_empty_trim_chars(self):
        self.assertEqual(trim_characters("hello", ""), "hello")

    def test_interspersed_chars(self):
        self.assertEqual(trim_characters("-h-e-l-l-o-", "-"), "h-e-l-l-o")

    def test_docstring_examples(self):
        self.assertEqual(trim_characters("-.-hello-.-", "-."), "hello")
        self.assertEqual(trim_characters("123abc123", "123"), "abc")


if __name__ == "__main__":
    unittest.main()
