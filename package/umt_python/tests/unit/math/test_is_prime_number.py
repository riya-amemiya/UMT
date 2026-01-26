import unittest

from src.math import is_prime_number


class TestIsPrimeNumber(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_prime_number(2))
        self.assertTrue(is_prime_number(17))
        self.assertFalse(is_prime_number(4))
        self.assertFalse(is_prime_number(1))
        self.assertFalse(is_prime_number(-3))

    def test_edge_cases(self):
        self.assertFalse(is_prime_number(0))
        self.assertTrue(is_prime_number(3))
        self.assertTrue(is_prime_number(97))

    def test_docstring_example(self):
        self.assertTrue(is_prime_number(2))
        self.assertTrue(is_prime_number(17))
        self.assertFalse(is_prime_number(4))
        self.assertFalse(is_prime_number(1))
        self.assertFalse(is_prime_number(-3))


if __name__ == "__main__":
    unittest.main()
