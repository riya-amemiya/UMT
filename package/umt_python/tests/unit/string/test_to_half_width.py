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


if __name__ == "__main__":
    unittest.main()
