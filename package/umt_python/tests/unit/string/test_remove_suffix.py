import unittest

from src.string import remove_suffix


class TestRemoveSuffix(unittest.TestCase):
    def test_removes_when_present(self):
        self.assertEqual(remove_suffix("file.txt", ".txt"), "file")

    def test_unchanged_when_absent(self):
        self.assertEqual(remove_suffix("file", ".txt"), "file")

    def test_removes_only_once(self):
        self.assertEqual(remove_suffix("path//", "/"), "path/")

    def test_empty_suffix(self):
        self.assertEqual(remove_suffix("abc", ""), "abc")

    def test_empty_input(self):
        self.assertEqual(remove_suffix("", "x"), "")


if __name__ == "__main__":
    unittest.main()
