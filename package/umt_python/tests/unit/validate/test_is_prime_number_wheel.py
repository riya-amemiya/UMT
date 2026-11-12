import unittest

from src.validate import is_prime_number


def _is_prime_by_trial_division(n: int) -> bool:
    if n < 2:
        return False
    return all(n % divisor != 0 for divisor in range(2, int(n**0.5) + 1))


class TestIsPrimeNumberWheel(unittest.TestCase):
    def test_matches_trial_division_for_small_numbers(self):
        for n in range(-10, 5000):
            self.assertEqual(is_prime_number(n), _is_prime_by_trial_division(n), n)

    def test_small_primes(self):
        self.assertTrue(is_prime_number(2))
        self.assertTrue(is_prime_number(3))

    def test_multiples_of_two_and_three(self):
        self.assertFalse(is_prime_number(4))
        self.assertFalse(is_prime_number(9))
        self.assertFalse(is_prime_number(1_000_002))

    def test_squares_of_primes(self):
        self.assertFalse(is_prime_number(25))
        self.assertFalse(is_prime_number(49))
        self.assertFalse(is_prime_number(121))
        self.assertFalse(is_prime_number(169))
        self.assertFalse(is_prime_number(1_000_003**2))

    def test_products_of_twin_candidates(self):
        self.assertFalse(is_prime_number(5 * 7))
        self.assertFalse(is_prime_number(11 * 13))
        self.assertFalse(is_prime_number(7 * 11))

    def test_large_primes(self):
        self.assertTrue(is_prime_number(2_147_483_647))
        self.assertTrue(is_prime_number(4_294_967_291))

    def test_integer_valued_floats(self):
        self.assertTrue(is_prime_number(13.0))
        self.assertFalse(is_prime_number(15.0))

    def test_non_integer_and_non_finite_floats(self):
        self.assertFalse(is_prime_number(2.5))
        self.assertFalse(is_prime_number(float("nan")))
        self.assertFalse(is_prime_number(float("inf")))

    def test_non_numeric_values(self):
        self.assertFalse(is_prime_number("7"))  # type: ignore[arg-type]
        self.assertFalse(is_prime_number(True))


if __name__ == "__main__":
    unittest.main()
