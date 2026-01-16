import unittest

from src import has_no_letters


class TestHasNoLetters(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(has_no_letters("12345"))
        self.assertTrue(has_no_letters("!@#$%^"))
        self.assertFalse(has_no_letters("abcde"))
        self.assertFalse(has_no_letters("Hello World"))

    def test_mixed_content(self):
        self.assertFalse(has_no_letters("abc123"))
        self.assertFalse(has_no_letters("123abc"))
        self.assertFalse(has_no_letters("!@#abc"))
        self.assertTrue(has_no_letters("123 !@#"))

    def test_unicode_characters(self):
        self.assertTrue(has_no_letters("🌟123#"))
        self.assertFalse(has_no_letters("你好世界"))
        self.assertFalse(has_no_letters("Привет"))
        self.assertFalse(has_no_letters("مرحبا"))
        self.assertTrue(has_no_letters("😊👍🎉"))

    def test_edge_cases(self):
        self.assertTrue(has_no_letters(""))

    def test_docstring_examples(self):
        self.assertTrue(has_no_letters("123"))
        self.assertTrue(has_no_letters("🌟123#"))
        self.assertFalse(has_no_letters("abc123"))


if __name__ == "__main__":
    unittest.main()
