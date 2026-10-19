import unittest

from src import to_half_width


class TestToHalfWidth(unittest.TestCase):
    def test_full_width_alphanumeric(self):
        self.assertEqual(
            to_half_width("Ｈｅｌｌｏ Ｗｏｒｌｄ １２３"), "Hello World 123"
        )
        self.assertEqual(to_half_width("ＡＢＣｄｅｆ０１２３"), "ABCdef0123")

    def test_string_with_no_full_width_chars(self):
        self.assertEqual(to_half_width("Hello World 123"), "Hello World 123")

    def test_string_with_mixed_chars(self):
        self.assertEqual(
            to_half_width("Ｈello Ｗorld １２３ and abc"), "Hello World 123 and abc"
        )

    def test_full_width_symbols_and_space(self):
        # Note: Current implementation only handles U+FF01 to U+FF5E and U+3000 (full-width space)
        self.assertEqual(
            to_half_width("！＂＃＄％＆＇（）＊＋，－．／"), "!\"#$%&'()*+,-./"
        )  # subset of symbols
        self.assertEqual(to_half_width("　"), " ")  # Full-width space

    def test_empty_string(self):
        self.assertEqual(to_half_width(""), "")

    def test_docstring_examples(self):
        self.assertEqual(
            to_half_width("Ｈｅｌｌｏ Ｗｏｒｌｄ １２３"), "Hello World 123"
        )
        self.assertEqual(to_half_width("ＡＢＣｄｅｆ"), "ABCdef")
        self.assertEqual(
            to_half_width("！＂＃＄％＆＇（）＊＋，－．／"), "!\"#$%&'()*+,-./"
        )
        self.assertEqual(to_half_width("　"), " ")

    def test_ts_full_width_digits_and_letters(self):
        self.assertEqual(to_half_width("０１２３４５６７８９"), "0123456789")
        self.assertEqual(to_half_width("ＡＢＣＤＥＦＧＨＩＪ"), "ABCDEFGHIJ")
        self.assertEqual(to_half_width("ａｂｃｄｅｆｇｈｉｊ"), "abcdefghij")
        self.assertEqual(to_half_width("ＡＢＣabc１２３123"), "ABCabc123123")

    def test_cjk_and_half_width_punctuation_kept(self):
        self.assertEqual(to_half_width("漢字カタカナ、。"), "漢字カタカナ、。")
        self.assertEqual(to_half_width("Hello 世界 123"), "Hello 世界 123")
        self.assertEqual(to_half_width("!@# $%^"), "!@# $%^")
        self.assertEqual(to_half_width("   "), "   ")

    def test_mixed_half_and_full_width(self):
        self.assertEqual(
            to_half_width("ＡＢＣａｂｃ１２３123"),
            "ABCabc123123",
        )
        self.assertEqual(to_half_width("Ｈｅｌｌｏ世界１２３"), "Hello世界123")

    def test_already_half_width_unchanged(self):
        self.assertEqual(to_half_width("ABCdef0123"), "ABCdef0123")
        self.assertEqual(to_half_width("!\"#$%&'()*+,-./"), "!\"#$%&'()*+,-./")

    def test_all_full_width_ascii_and_ideographic_space(self):
        full_width_ascii = "".join(chr(i) for i in range(0xFF01, 0xFF5F))
        expected_ascii = "".join(chr(i - 0xFEE0) for i in range(0xFF01, 0xFF5F))
        self.assertEqual(to_half_width(full_width_ascii), expected_ascii)
        self.assertEqual(to_half_width("\u3000"), " ")
        self.assertEqual(
            to_half_width(full_width_ascii + "\u3000"), expected_ascii + " "
        )

    def test_code_point_offset(self):
        for character in "０Ａａ！～":
            self.assertEqual(
                to_half_width(character),
                chr(ord(character) - 0xFEE0),
            )
        self.assertEqual(to_half_width("\u3000"), "\u0020")


if __name__ == "__main__":
    unittest.main()
