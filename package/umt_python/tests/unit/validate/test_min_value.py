import unittest

from src.validate import min_value, number


class TestMinValue(unittest.TestCase):
    def test_returns_true_for_values_at_or_above_the_minimum(self):
        rule = min_value(3)
        self.assertTrue(rule.validate(3))
        self.assertTrue(rule.validate(3.5))
        self.assertTrue(rule.validate(100))

    def test_returns_false_for_values_below_the_minimum(self):
        rule = min_value(3)
        self.assertFalse(rule.validate(2))
        self.assertFalse(rule.validate(2.999))
        self.assertFalse(rule.validate(-3))

    def test_returns_false_for_nan(self):
        self.assertFalse(min_value(3).validate(float("nan")))

    def test_carries_the_custom_message(self):
        self.assertEqual(min_value(3, "too small").message, "too small")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(min_value(3).message, "")

    def test_composes_with_number(self):
        validator = number([min_value(3, "too small")])
        self.assertTrue(validator(5).validate)
        self.assertEqual(validator(2).message, "too small")


if __name__ == "__main__":
    unittest.main()
