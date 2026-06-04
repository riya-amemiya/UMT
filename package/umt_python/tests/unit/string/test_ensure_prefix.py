import unittest

from src.string import ensure_prefix


class TestEnsurePrefix(unittest.TestCase):
    def test_prepends_when_missing(self):
        self.assertEqual(
            ensure_prefix("example.com", "https://"), "https://example.com"
        )

    def test_does_not_duplicate(self):
        self.assertEqual(
            ensure_prefix("https://example.com", "https://"), "https://example.com"
        )

    def test_empty_input(self):
        self.assertEqual(ensure_prefix("", "/"), "/")

    def test_empty_prefix(self):
        self.assertEqual(ensure_prefix("abc", ""), "abc")


if __name__ == "__main__":
    unittest.main()
