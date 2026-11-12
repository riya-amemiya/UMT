import unittest

from src.string import levenshtein_distance


class TestLevenshteinDistance(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(levenshtein_distance("kitten", "sitting"), 3)
        self.assertEqual(levenshtein_distance("hello", "hello"), 0)
        self.assertEqual(levenshtein_distance("", "abc"), 3)

    def test_edge_cases(self):
        self.assertEqual(levenshtein_distance("", ""), 0)
        self.assertEqual(levenshtein_distance("abc", ""), 3)

    def test_docstring_example(self):
        self.assertEqual(levenshtein_distance("kitten", "sitting"), 3)
        self.assertEqual(levenshtein_distance("hello", "hello"), 0)
        self.assertEqual(levenshtein_distance("", "abc"), 3)


class TestLevenshteinDistanceEqualInputs(unittest.TestCase):
    def test_identical_strings(self):
        self.assertEqual(levenshtein_distance("same text", "same text"), 0)
        self.assertEqual(levenshtein_distance("", ""), 0)

    def test_first_string_shorter(self):
        self.assertEqual(levenshtein_distance("abc", "abcd"), 1)

    def test_first_string_longer(self):
        self.assertEqual(levenshtein_distance("abcd", "abc"), 1)

    def test_second_string_empty(self):
        self.assertEqual(levenshtein_distance("abc", ""), 3)


if __name__ == "__main__":
    unittest.main()
