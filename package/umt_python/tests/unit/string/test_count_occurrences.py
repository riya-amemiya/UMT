import unittest

from src.string import count_occurrences


class TestCountOccurrences(unittest.TestCase):
    def test_non_overlapping_occurrences(self):
        self.assertEqual(count_occurrences("ababab", "ab"), 3)

    def test_does_not_count_overlapping(self):
        self.assertEqual(count_occurrences("aaaa", "aa"), 2)
        self.assertEqual(count_occurrences("aaa", "aa"), 1)

    def test_single_character(self):
        self.assertEqual(count_occurrences("banana", "a"), 3)

    def test_absent_substring(self):
        self.assertEqual(count_occurrences("hello", "z"), 0)

    def test_empty_search(self):
        self.assertEqual(count_occurrences("hello", ""), 0)
        self.assertEqual(count_occurrences("", ""), 0)

    def test_empty_input(self):
        self.assertEqual(count_occurrences("", "a"), 0)

    def test_search_equals_whole_string(self):
        self.assertEqual(count_occurrences("hello", "hello"), 1)

    def test_search_longer_than_input(self):
        self.assertEqual(count_occurrences("ab", "abcd"), 0)

    def test_unicode(self):
        self.assertEqual(count_occurrences("こんにちはこんにちは", "ん"), 2)
        self.assertEqual(count_occurrences("😀😀😁", "😀"), 2)
        self.assertEqual(count_occurrences("café cafe", "é"), 1)

    def test_docstring_examples(self):
        self.assertEqual(count_occurrences("ababab", "ab"), 3)
        self.assertEqual(count_occurrences("aaaa", "aa"), 2)
        self.assertEqual(count_occurrences("banana", "a"), 3)
        self.assertEqual(count_occurrences("hello", ""), 0)

    def test_matches_find_loop_path(self):
        samples = [
            ("ababab", "ab"),
            ("aaaa", "aa"),
            ("aaa", "aa"),
            ("banana", "a"),
            ("hello", "z"),
            ("hello", ""),
            ("hello", "hello"),
            ("", "a"),
            ("", ""),
            ("ab", "abcd"),
            ("a" * 32, "a"),
            ("ab" * 50, "ab"),
            ("prefix-and-more", "and"),
            ("こんにちはこんにちは", "ん"),
            ("😀😀😁", "😀"),
            ("The quick brown fox jumps over the lazy dog", " "),
            ("The quick brown fox jumps over the lazy dog", "fox"),
        ]
        for haystack, needle in samples:
            with self.subTest(haystack=haystack, needle=needle):
                self.assertEqual(
                    count_occurrences(haystack, needle),
                    _find_loop_count(haystack, needle),
                )


def _find_loop_count(string_: str, search: str) -> int:
    if search == "":
        return 0
    count = 0
    index = string_.find(search)
    while index != -1:
        count += 1
        index = string_.find(search, index + len(search))
    return count


if __name__ == "__main__":
    unittest.main()
