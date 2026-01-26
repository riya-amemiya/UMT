import unittest

from src.string import truncate


class TestTruncate(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(truncate("Hello World", 5), "Hello...")
        self.assertEqual(truncate("Hello World", 5, "~"), "Hello~")
        self.assertEqual(truncate("Hello", 10), "Hello")

    def test_edge_cases(self):
        self.assertEqual(truncate("", 5), "")
        self.assertEqual(truncate("Hello", 5), "Hello")

    def test_invalid_length(self):
        with self.assertRaises(ValueError):
            truncate("Hello", -1)

    def test_docstring_example(self):
        self.assertEqual(truncate("Hello World", 5), "Hello...")
        self.assertEqual(truncate("Hello World", 5, "~"), "Hello~")
        self.assertEqual(truncate("Hello", 10), "Hello")


if __name__ == "__main__":
    unittest.main()
