import unittest

from src.validate import is_prime_number


class TestIsPrimeNumber(unittest.TestCase):
    def test_less_than_or_equal_to_1(self):
        self.assertFalse(is_prime_number(0))
        self.assertFalse(is_prime_number(1))

    def test_prime_numbers(self):
        self.assertTrue(is_prime_number(2))
        self.assertTrue(is_prime_number(3))
        self.assertTrue(is_prime_number(5))
        self.assertTrue(is_prime_number(7))
        self.assertTrue(is_prime_number(11))
        self.assertTrue(is_prime_number(13))
        self.assertTrue(is_prime_number(17))
        self.assertTrue(is_prime_number(19))
        self.assertTrue(is_prime_number(23))
        self.assertTrue(is_prime_number(29))

    def test_composite_numbers(self):
        self.assertFalse(is_prime_number(4))
        self.assertFalse(is_prime_number(6))
        self.assertFalse(is_prime_number(8))
        self.assertFalse(is_prime_number(9))
        self.assertFalse(is_prime_number(10))
        self.assertFalse(is_prime_number(12))
        self.assertFalse(is_prime_number(14))
        self.assertFalse(is_prime_number(15))
        self.assertFalse(is_prime_number(16))
        self.assertFalse(is_prime_number(18))

    def test_negative_numbers(self):
        self.assertFalse(is_prime_number(-2))
        self.assertFalse(is_prime_number(-7))
        self.assertFalse(is_prime_number(-11))

    def test_non_integer_numbers(self):
        self.assertFalse(is_prime_number(2.5))
        self.assertFalse(is_prime_number(3.14))
        self.assertTrue(is_prime_number(7.0))  # 7.0 is treated as integer 7

    def test_large_non_prime_numbers(self):
        large_non_prime_number = 10_000_000_000_000  # 10^13
        self.assertFalse(is_prime_number(large_non_prime_number))

    def test_large_prime_numbers(self):
        large_prime_number = 982_451_653  # 982,451,653 is a known prime number
        self.assertTrue(is_prime_number(large_prime_number))

    def test_boolean(self):
        self.assertFalse(is_prime_number(True))
        self.assertFalse(is_prime_number(False))


if __name__ == "__main__":
    unittest.main()
