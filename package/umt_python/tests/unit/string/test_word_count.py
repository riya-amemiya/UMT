import unittest

from src.string import word_count


class TestWordCount(unittest.TestCase):
    def test_space_separated_words(self):
        self.assertEqual(word_count("hello world"), 2)

    def test_camel_case_boundaries(self):
        self.assertEqual(word_count("camelCaseSplit"), 3)

    def test_empty_string(self):
        self.assertEqual(word_count(""), 0)

    def test_words_across_punctuation(self):
        self.assertEqual(word_count("foo, bar! baz?"), 3)


if __name__ == "__main__":
    unittest.main()
