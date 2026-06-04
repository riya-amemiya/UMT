import unittest

from src.string import remove_prefix


class TestRemovePrefix(unittest.TestCase):
    def test_removes_when_present(self):
        self.assertEqual(
            remove_prefix("https://example.com", "https://"), "example.com"
        )

    def test_unchanged_when_absent(self):
        self.assertEqual(remove_prefix("example.com", "https://"), "example.com")

    def test_removes_only_once(self):
        self.assertEqual(remove_prefix("//path", "/"), "/path")

    def test_empty_prefix(self):
        self.assertEqual(remove_prefix("abc", ""), "abc")

    def test_empty_input(self):
        self.assertEqual(remove_prefix("", "x"), "")


if __name__ == "__main__":
    unittest.main()
