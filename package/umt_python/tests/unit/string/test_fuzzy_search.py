import unittest

from src.string import fuzzy_search


class TestFuzzySearch(unittest.TestCase):
    def test_basic_cases(self):
        result = fuzzy_search("hello", ["hello", "world", "helo", "help"])
        self.assertEqual(len(result), 3)
        self.assertEqual(result[0]["item"], "hello")
        self.assertEqual(result[0]["score"], 1.0)

    def test_edge_cases(self):
        self.assertEqual(fuzzy_search("", ["hello", "world"]), [])
        result = fuzzy_search("xyz", ["hello", "world"], threshold=0.0)
        self.assertEqual(len(result), 2)

    def test_threshold(self):
        result = fuzzy_search("hello", ["hello", "helo", "help"], threshold=0.9)
        self.assertEqual(len(result), 1)
        self.assertEqual(result[0]["item"], "hello")

    def test_docstring_example(self):
        result = fuzzy_search("hello", ["hello", "world", "helo", "help"])
        self.assertEqual(result[0]["item"], "hello")
        self.assertEqual(result[0]["score"], 1.0)


if __name__ == "__main__":
    unittest.main()
