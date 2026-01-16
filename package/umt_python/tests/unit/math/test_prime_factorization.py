import unittest

from src.math import prime_factorization


class TestPrimeFactorization(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(
            prime_factorization(12),
            [{"number": 2, "count": 2}, {"number": 3, "count": 1}],
        )
        self.assertEqual(prime_factorization(7), [{"number": 7, "count": 1}])

    def test_edge_cases(self):
        self.assertEqual(
            prime_factorization(100),
            [{"number": 2, "count": 2}, {"number": 5, "count": 2}],
        )

    def test_docstring_example(self):
        self.assertEqual(
            prime_factorization(12),
            [{"number": 2, "count": 2}, {"number": 3, "count": 1}],
        )


if __name__ == "__main__":
    unittest.main()
