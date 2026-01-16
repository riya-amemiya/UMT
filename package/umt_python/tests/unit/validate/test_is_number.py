import unittest

from src.validate import is_number


class TestIsNumber(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_number(0.1))
        self.assertTrue(is_number(42))
        self.assertTrue(is_number(-5))
        self.assertTrue(is_number(0))

    def test_loose_mode(self):
        self.assertTrue(is_number("0.1"))
        self.assertTrue(is_number("42"))
        self.assertFalse(is_number("0.1", False))

    def test_invalid_values(self):
        self.assertFalse(is_number(None))
        self.assertFalse(is_number([]))
        self.assertFalse(is_number({}))
        self.assertFalse(is_number(True))
        self.assertFalse(is_number("abc"))

    def test_edge_cases(self):
        self.assertFalse(is_number(float("inf")))
        self.assertFalse(is_number(float("nan")))

    def test_docstring_example(self):
        self.assertTrue(is_number(0.1))
        self.assertTrue(is_number("0.1"))
        self.assertFalse(is_number("0.1", False))


if __name__ == "__main__":
    unittest.main()
