import unittest

from src.string import count_occurrences


class TestCountOccurrences(unittest.TestCase):
    def test_non_overlapping_occurrences(self):
        self.assertEqual(count_occurrences("ababab", "ab"), 3)

    def test_does_not_count_overlapping(self):
        self.assertEqual(count_occurrences("aaaa", "aa"), 2)

    def test_single_character(self):
        self.assertEqual(count_occurrences("banana", "a"), 3)

    def test_absent_substring(self):
        self.assertEqual(count_occurrences("hello", "z"), 0)

    def test_empty_search(self):
        self.assertEqual(count_occurrences("hello", ""), 0)

    def test_empty_input(self):
        self.assertEqual(count_occurrences("", "a"), 0)


if __name__ == "__main__":
    unittest.main()
