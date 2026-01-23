import unittest

from src.array import compare_function_default


class TestCompareFunctionDefault(unittest.TestCase):
    def test_greater_than(self):
        """Test when a > b returns 1."""
        self.assertEqual(compare_function_default(2, 1), 1)
        self.assertEqual(compare_function_default(100, 50), 1)
        self.assertEqual(compare_function_default(1.5, 1.0), 1)

    def test_less_than(self):
        """Test when a < b returns -1."""
        self.assertEqual(compare_function_default(1, 2), -1)
        self.assertEqual(compare_function_default(50, 100), -1)
        self.assertEqual(compare_function_default(1.0, 1.5), -1)

    def test_equal(self):
        """Test when a == b returns 0."""
        self.assertEqual(compare_function_default(2, 2), 0)
        self.assertEqual(compare_function_default(0, 0), 0)
        self.assertEqual(compare_function_default(1.5, 1.5), 0)

    def test_string_comparison(self):
        """Test string comparison."""
        self.assertEqual(compare_function_default("b", "a"), 1)
        self.assertEqual(compare_function_default("a", "b"), -1)
        self.assertEqual(compare_function_default("a", "a"), 0)

    def test_negative_numbers(self):
        """Test negative number comparison."""
        self.assertEqual(compare_function_default(-1, -2), 1)
        self.assertEqual(compare_function_default(-2, -1), -1)
        self.assertEqual(compare_function_default(-1, -1), 0)

    def test_docstring_examples(self):
        """Test examples from docstring."""
        self.assertEqual(compare_function_default(2, 1), 1)
        self.assertEqual(compare_function_default(1, 2), -1)
        self.assertEqual(compare_function_default(2, 2), 0)
        self.assertEqual(compare_function_default("b", "a"), 1)
        self.assertEqual(compare_function_default("a", "b"), -1)


if __name__ == "__main__":
    unittest.main()
