import unittest

from src.string import ensure_suffix


class TestEnsureSuffix(unittest.TestCase):
    def test_appends_when_missing(self):
        self.assertEqual(ensure_suffix("file", ".txt"), "file.txt")

    def test_does_not_duplicate(self):
        self.assertEqual(ensure_suffix("file.txt", ".txt"), "file.txt")

    def test_empty_input(self):
        self.assertEqual(ensure_suffix("", "/"), "/")

    def test_empty_suffix(self):
        self.assertEqual(ensure_suffix("abc", ""), "abc")


if __name__ == "__main__":
    unittest.main()
