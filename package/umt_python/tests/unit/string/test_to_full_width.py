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

    def test_docstring_examples(self):
        self.assertEqual(to_full_width("Abc123"), "Ａｂｃ１２３")
        self.assertEqual(to_full_width("a-b!"), "ａ-ｂ!")
        self.assertEqual(to_full_width(""), "")

    def test_already_full_width_unchanged(self):
        self.assertEqual(to_full_width("Ａｂｃ１２３"), "Ａｂｃ１２３")

    def test_mixed_half_and_full_width(self):
        self.assertEqual(to_full_width("ABCabc１２３123"), "ＡＢＣａｂｃ１２３１２３")

    def test_cjk_and_symbols_kept(self):
        self.assertEqual(to_full_width("漢字カタカナ、。"), "漢字カタカナ、。")
        self.assertEqual(to_full_width("Hello 世界 123"), "Ｈｅｌｌｏ 世界 １２３")

    def test_punctuation_and_space_only(self):
        self.assertEqual(to_full_width("!@# $%^"), "!@# $%^")
        self.assertEqual(to_full_width("   "), "   ")

    def test_all_ascii_alnum(self):
        ascii_alnum = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        expected = "".join(chr(ord(character) + 0xFEE0) for character in ascii_alnum)
        self.assertEqual(to_full_width(ascii_alnum), expected)

    def test_code_point_offset(self):
        for character in "0Aaz9Z":
            self.assertEqual(
                to_full_width(character),
                chr(ord(character) + 0xFEE0),
            )


if __name__ == "__main__":
    unittest.main()
