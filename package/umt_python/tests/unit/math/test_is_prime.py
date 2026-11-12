import unittest

from src.math import is_prime


class TestIsPrime(unittest.TestCase):
    def test_below_two(self):
        self.assertFalse(is_prime(1))
        self.assertFalse(is_prime(0))
        self.assertFalse(is_prime(-7))

    def test_two_and_three(self):
        self.assertTrue(is_prime(2))
        self.assertTrue(is_prime(3))

    def test_multiples_of_two_or_three(self):
        self.assertFalse(is_prime(4))
        self.assertFalse(is_prime(9))
        self.assertFalse(is_prime(100))

    def test_primes(self):
        for n in (5, 7, 29, 97, 7919, 2_147_483_647):
            self.assertTrue(is_prime(n))

    def test_composites_divisible_by_6k_minus_1(self):
        self.assertFalse(is_prime(25))
        self.assertFalse(is_prime(121))

    def test_composites_divisible_by_6k_plus_1(self):
        self.assertFalse(is_prime(49))
        self.assertFalse(is_prime(91))
        self.assertFalse(is_prime(169))

    def test_integral_floats(self):
        self.assertTrue(is_prime(7.0))
        self.assertFalse(is_prime(8.0))

    def test_non_integers(self):
        self.assertFalse(is_prime(7.5))
        self.assertFalse(is_prime(2.000001))

    def test_nan_and_infinity(self):
        self.assertFalse(is_prime(float("nan")))
        self.assertFalse(is_prime(float("inf")))
        self.assertFalse(is_prime(float("-inf")))

    def test_booleans(self):
        self.assertFalse(is_prime(True))
        self.assertFalse(is_prime(False))

    def test_float_above_two_pow_53(self):
        self.assertFalse(is_prime(float(2**53 + 2)))


if __name__ == "__main__":
    unittest.main()
