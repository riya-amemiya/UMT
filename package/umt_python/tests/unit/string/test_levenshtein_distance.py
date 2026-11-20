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

    def test_single_character_edits(self):
        self.assertEqual(levenshtein_distance("cat", "bat"), 1)
        self.assertEqual(levenshtein_distance("cat", "cats"), 1)
        self.assertEqual(levenshtein_distance("cats", "cat"), 1)

    def test_multiple_differences(self):
        self.assertEqual(levenshtein_distance("saturday", "sunday"), 3)
        self.assertEqual(levenshtein_distance("hello", "world"), 4)
        self.assertEqual(levenshtein_distance("abc", "xyz"), 3)

    def test_case_sensitive(self):
        self.assertEqual(levenshtein_distance("Hello", "hello"), 1)
        self.assertEqual(levenshtein_distance("ABC", "abc"), 3)

    def test_unicode(self):
        self.assertEqual(levenshtein_distance("café", "cafe"), 1)
        self.assertEqual(levenshtein_distance("😀", "😁"), 1)
        self.assertEqual(levenshtein_distance("こんにちは", "こんばんは"), 2)

    def test_matches_builtin_min_path(self):
        samples = [
            ("kitten", "sitting"),
            ("", "hello"),
            ("hello", ""),
            ("a" * 32, "b" * 32),
            ("prefix", "prefix-and-more"),
            ("and-more-suffix", "suffix"),
        ]
        for left, right in samples:
            with self.subTest(left=left, right=right):
                self.assertEqual(
                    levenshtein_distance(left, right),
                    _min_path_distance(left, right),
                )


def _min_path_distance(string1: str, string2: str) -> int:
    if string1 == string2:
        return 0
    if len(string1) < len(string2):
        string1, string2 = string2, string1
    if not string2:
        return len(string1)
    previous_row = list(range(len(string2) + 1))
    current_row = [0] * (len(string2) + 1)
    for i, char1 in enumerate(string1):
        current_row[0] = i + 1
        for j, char2 in enumerate(string2):
            current_row[j + 1] = min(
                previous_row[j + 1] + 1,
                current_row[j] + 1,
                previous_row[j] + (0 if char1 == char2 else 1),
            )
        previous_row, current_row = current_row, previous_row
    return previous_row[len(string2)]


if __name__ == "__main__":
    unittest.main()
