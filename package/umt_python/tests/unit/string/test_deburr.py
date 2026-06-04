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


if __name__ == "__main__":
    unittest.main()
