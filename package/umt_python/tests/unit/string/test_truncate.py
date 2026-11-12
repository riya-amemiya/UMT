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

    def test_negative_length_input(self):
        self.assertEqual(truncate("Hello", -1), "Hell...")

    def test_docstring_example(self):
        self.assertEqual(truncate("Hello World", 5), "Hello...")
        self.assertEqual(truncate("Hello World", 5, "~"), "Hello~")
        self.assertEqual(truncate("Hello", 10), "Hello")


class TestTruncateNegativeLength(unittest.TestCase):
    def test_negative_length_slices_from_end(self):
        self.assertEqual(truncate("Hello", -1), "Hell...")

    def test_negative_length_beyond_string(self):
        self.assertEqual(truncate("Hello", -10), "...")
        self.assertEqual(truncate("", -1), "...")

    def test_zero_length(self):
        self.assertEqual(truncate("Hello", 0), "...")


if __name__ == "__main__":
    unittest.main()
