import unittest

from src.string import to_full_width


class TestToFullWidth(unittest.TestCase):
    def test_digits(self):
        self.assertEqual(to_full_width("123"), "１２３")

    def test_uppercase_letters(self):
        self.assertEqual(to_full_width("ABC"), "ＡＢＣ")

    def test_lowercase_letters(self):
        self.assertEqual(to_full_width("abc"), "ａｂｃ")

    def test_mixed_alphanumeric(self):
        self.assertEqual(to_full_width("Abc123"), "Ａｂｃ１２３")

    def test_non_alphanumeric_unchanged(self):
        self.assertEqual(to_full_width("a-b!"), "ａ-ｂ!")

    def test_empty_string(self):
        self.assertEqual(to_full_width(""), "")


if __name__ == "__main__":
    unittest.main()
