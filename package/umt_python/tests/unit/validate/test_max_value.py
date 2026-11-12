import unittest

from src.validate import max_value, number


class TestMaxValue(unittest.TestCase):
    def test_returns_true_for_values_at_or_below_the_maximum(self):
        rule = max_value(10)
        self.assertTrue(rule.validate(10))
        self.assertTrue(rule.validate(9.5))
        self.assertTrue(rule.validate(-100))

    def test_returns_false_for_values_above_the_maximum(self):
        rule = max_value(10)
        self.assertFalse(rule.validate(11))
        self.assertFalse(rule.validate(10.001))

    def test_returns_false_for_nan(self):
        self.assertFalse(max_value(10).validate(float("nan")))

    def test_carries_the_custom_message(self):
        self.assertEqual(max_value(10, "too large").message, "too large")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(max_value(10).message, "")

    def test_composes_with_number(self):
        validator = number([max_value(10, "too large")])
        self.assertTrue(validator(5).validate)
        self.assertEqual(validator(11).message, "too large")


if __name__ == "__main__":
    unittest.main()
