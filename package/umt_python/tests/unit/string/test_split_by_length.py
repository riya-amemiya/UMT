import unittest

from src.string import split_by_length


class TestSplitByLength(unittest.TestCase):
    def test_equal_chunks(self):
        self.assertEqual(split_by_length("abcdef", 2), ["ab", "cd", "ef"])

    def test_remainder_in_last_chunk(self):
        self.assertEqual(split_by_length("abcdefg", 3), ["abc", "def", "g"])

    def test_single_chunk_when_length_exceeds(self):
        self.assertEqual(split_by_length("abc", 10), ["abc"])

    def test_surrogate_pair_safe(self):
        self.assertEqual(
            split_by_length("\U0001f600\U0001f601\U0001f602\U0001f603", 2),
            ["\U0001f600\U0001f601", "\U0001f602\U0001f603"],
        )

    def test_empty_string(self):
        self.assertEqual(split_by_length("", 3), [])


if __name__ == "__main__":
    unittest.main()
