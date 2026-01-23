import unittest

from src.math import repeated_trial


class TestRepeatedTrial(unittest.TestCase):
    def test_basic_case(self):
        result = repeated_trial(5, 2, {"x": 1, "y": 3})
        self.assertEqual(result, [80.0, 243.0])

    def test_different_probability(self):
        result = repeated_trial(3, 1, {"x": 1, "y": 2})

        self.assertIsInstance(result, list)
        self.assertEqual(len(result), 2)

    def test_zero_successes(self):
        result = repeated_trial(3, 0, {"x": 1, "y": 2})
        self.assertIsInstance(result, list)

    def test_all_successes(self):
        result = repeated_trial(3, 3, {"x": 1, "y": 2})
        self.assertIsInstance(result, list)

    def test_docstring_example(self):
        self.assertEqual(repeated_trial(5, 2, {"x": 1, "y": 3}), [80.0, 243.0])


if __name__ == "__main__":
    unittest.main()
