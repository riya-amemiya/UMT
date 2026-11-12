import unittest

from src.string import damerau_levenshtein_distance, levenshtein_distance


class TestDamerauLevenshteinDistance(unittest.TestCase):
    def test_docstring_examples(self):
        self.assertEqual(damerau_levenshtein_distance("ca", "ac"), 1)
        self.assertEqual(damerau_levenshtein_distance("kitten", "sitting"), 3)
        self.assertEqual(damerau_levenshtein_distance("ca", "abc"), 3)

    def test_identical(self):
        self.assertEqual(damerau_levenshtein_distance("hello", "hello"), 0)
        self.assertEqual(damerau_levenshtein_distance("", ""), 0)

    def test_empty(self):
        self.assertEqual(damerau_levenshtein_distance("", "abc"), 3)
        self.assertEqual(damerau_levenshtein_distance("abc", ""), 3)

    def test_adjacent_transposition(self):
        self.assertEqual(damerau_levenshtein_distance("ab", "ba"), 1)
        self.assertEqual(levenshtein_distance("ab", "ba"), 2)
        self.assertEqual(damerau_levenshtein_distance("abcd", "acbd"), 1)
        self.assertEqual(damerau_levenshtein_distance("abcdef", "badcfe"), 3)

    def test_single_edits(self):
        self.assertEqual(damerau_levenshtein_distance("a", "b"), 1)
        self.assertEqual(damerau_levenshtein_distance("cat", "cats"), 1)
        self.assertEqual(damerau_levenshtein_distance("cats", "cat"), 1)

    def test_transposition_not_cheaper(self):
        self.assertEqual(damerau_levenshtein_distance("aax", "aay"), 1)

    def test_symmetric(self):
        self.assertEqual(damerau_levenshtein_distance("abc", "ca"), 3)
        self.assertEqual(damerau_levenshtein_distance("sitting", "kitten"), 3)

    def test_compares_code_points(self):
        self.assertEqual(damerau_levenshtein_distance("\U0001f600x", "x\U0001f600"), 1)


if __name__ == "__main__":
    unittest.main()
