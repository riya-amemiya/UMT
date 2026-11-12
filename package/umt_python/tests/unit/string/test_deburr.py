import unittest

from src.string import deburr


class TestDeburr(unittest.TestCase):
    def test_removes_acute_accents(self):
        self.assertEqual(deburr("déjà vu"), "deja vu")

    def test_removes_multiple_marks(self):
        self.assertEqual(deburr("Crème brûlée"), "Creme brulee")

    def test_mix_of_accented_and_plain(self):
        self.assertEqual(deburr("résumé café"), "resume cafe")

    def test_plain_ascii_unchanged(self):
        self.assertEqual(deburr("hello world"), "hello world")

    def test_empty_string(self):
        self.assertEqual(deburr(""), "")


class TestDeburrDiacriticProperty(unittest.TestCase):
    def test_removes_spacing_diacritics(self):
        self.assertEqual(deburr("a^b`c"), "abc")
        self.assertEqual(deburr("x\u00a8y\u00b4z"), "xyz")

    def test_removes_kana_sound_marks(self):
        self.assertEqual(deburr("\u30e9\u30fc\u30e1\u30f3"), "\u30e9\u30e1\u30f3")
        self.assertEqual(deburr("\u304c\u304e"), "\u304b\u304d")

    def test_keeps_combining_marks_without_diacritic_property(self):
        self.assertEqual(deburr("\u0e01\u0e38"), "\u0e01\u0e38")
        self.assertEqual(deburr("e\u20d0"), "e\u20d0")

    def test_removes_supplementary_plane_diacritics(self):
        self.assertEqual(deburr("a\U0001e944b"), "ab")


if __name__ == "__main__":
    unittest.main()
