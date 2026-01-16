import unittest

from src.string import string_similarity


class TestStringSimilarity(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(string_similarity("hello", "hello"), 1.0)
        self.assertAlmostEqual(string_similarity("hello", "hallo"), 0.8)
        self.assertEqual(string_similarity("", "abc"), 0.0)

    def test_edge_cases(self):
        self.assertEqual(string_similarity("", ""), 1.0)
        self.assertEqual(string_similarity("abc", ""), 0.0)

    def test_docstring_example(self):
        self.assertEqual(string_similarity("hello", "hello"), 1.0)
        self.assertAlmostEqual(string_similarity("hello", "hallo"), 0.8)
        self.assertEqual(string_similarity("", "abc"), 0.0)


if __name__ == "__main__":
    unittest.main()
