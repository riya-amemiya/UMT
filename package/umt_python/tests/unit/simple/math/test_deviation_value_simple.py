import unittest

from src.simple.math import deviation_value_simple


class TestDeviationValueSimple(unittest.TestCase):
    def test_explicit_values(self):
        # When value equals average (no deviation)
        self.assertEqual(deviation_value_simple(50, 50, 10), 50)

        # One standard deviation above average
        self.assertEqual(deviation_value_simple(60, 50, 10), 60)

        # One standard deviation below average
        self.assertEqual(deviation_value_simple(40, 50, 10), 40)

        # When standard deviation is 0
        self.assertEqual(deviation_value_simple(100, 50, 0), 50)

    def test_missing_standard_deviation(self):
        with self.assertRaises(TypeError):
            deviation_value_simple(50, 50)  # type: ignore

    def test_array_input(self):
        # Using simple array [40, 50, 60]
        # mean = 50
        # population standard deviation ≈ 8.165
        scores = [40, 50, 60]
        self.assertAlmostEqual(deviation_value_simple(60, scores), 62.25, places=2)
        self.assertEqual(deviation_value_simple(50, scores), 50)
        self.assertAlmostEqual(deviation_value_simple(40, scores), 37.75, places=2)

    def test_same_reference_values(self):
        same_scores = [50, 50, 50]
        self.assertEqual(deviation_value_simple(50, same_scores), 50)
        self.assertEqual(deviation_value_simple(0, same_scores), 50)
        self.assertEqual(deviation_value_simple(100, same_scores), 50)


if __name__ == "__main__":
    unittest.main()
