import unittest

from src.ip import is_in_range


class TestIsInRange(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_in_range("192.168.1.100", "192.168.0.0", 16))
        self.assertFalse(is_in_range("10.0.0.1", "192.168.0.0", 16))

    def test_edge_cases(self):
        self.assertTrue(is_in_range("192.168.1.1", "192.168.1.1", 32))
        self.assertTrue(is_in_range("0.0.0.0", "0.0.0.0", 0))

    def test_invalid_input(self):
        with self.assertRaises(ValueError):
            is_in_range("", "192.168.0.0", 16)
        with self.assertRaises(ValueError):
            is_in_range("192.168.1.1", "", 16)
        with self.assertRaises(ValueError):
            is_in_range("192.168.1.1", "192.168.0.0", -1)

    def test_docstring_example(self):
        self.assertTrue(is_in_range("192.168.1.100", "192.168.0.0", 16))
        self.assertFalse(is_in_range("10.0.0.1", "192.168.0.0", 16))


if __name__ == "__main__":
    unittest.main()
