import unittest

from src.math import linear_congruential_generator


class TestLinearCongruentialGenerator(unittest.TestCase):
    def test_basic_case(self):
        result = linear_congruential_generator(12345)

        self.assertIsInstance(result, int)
        self.assertGreaterEqual(result, 0)

    def test_custom_params(self):
        result = linear_congruential_generator(12345, 1000, 10, 5)
        expected = (10 * 12345 + 5) % 1000
        self.assertEqual(result, expected)

    def test_reproducibility(self):
        result1 = linear_congruential_generator(12345)
        result2 = linear_congruential_generator(12345)
        self.assertEqual(result1, result2)

    def test_formula(self):
        seed = 100
        max_value = 1000
        multiplier = 7
        increment = 3
        expected = (multiplier * seed + increment) % max_value
        result = linear_congruential_generator(seed, max_value, multiplier, increment)
        self.assertEqual(result, expected)


if __name__ == "__main__":
    unittest.main()
