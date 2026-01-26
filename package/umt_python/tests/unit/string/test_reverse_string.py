import unittest

from src import reverse_string


class TestReverseString(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(reverse_string("hello"), "olleh")
        self.assertEqual(reverse_string("Python"), "nohtyP")

    def test_palindrome(self):
        self.assertEqual(reverse_string("madam"), "madam")

    def test_empty_string(self):
        self.assertEqual(reverse_string(""), "")

    def test_string_with_spaces(self):
        self.assertEqual(reverse_string("hello world"), "dlrow olleh")

    def test_string_with_numbers_and_symbols(self):
        self.assertEqual(reverse_string("123!@#"), "#@!321")

    def test_unicode_string(self):
        self.assertEqual(reverse_string("こんにちは"), "はちにんこ")

    def test_docstring_examples(self):
        self.assertEqual(reverse_string("Hello"), "olleH")
        self.assertEqual(reverse_string("madam"), "madam")


if __name__ == "__main__":
    unittest.main()
