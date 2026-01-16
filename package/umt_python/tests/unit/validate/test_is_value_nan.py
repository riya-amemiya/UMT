import unittest

from src.validate import is_value_nan


class TestIsValueNan(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_value_nan(float("nan")))
        self.assertFalse(is_value_nan(1))
        self.assertFalse(is_value_nan(0))

    def test_edge_cases(self):
        self.assertFalse(is_value_nan(float("inf")))
        self.assertFalse(is_value_nan(None))
        self.assertFalse(is_value_nan("nan"))


if __name__ == "__main__":
    unittest.main()
