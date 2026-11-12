import unittest

from src.validate import double, number


class TestDouble(unittest.TestCase):
    def test_returns_true_for_numbers_with_a_fractional_part(self):
        rule = double()
        self.assertTrue(rule.validate(1.5))
        self.assertTrue(rule.validate(-0.25))

    def test_returns_false_for_integers(self):
        rule = double()
        self.assertFalse(rule.validate(1))
        self.assertFalse(rule.validate(2.0))

    def test_returns_false_for_non_finite_numbers(self):
        rule = double()
        self.assertFalse(rule.validate(float("nan")))
        self.assertFalse(rule.validate(float("inf")))

    def test_carries_the_custom_message(self):
        self.assertEqual(double("must be a double").message, "must be a double")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(double().message, "")

    def test_composes_with_number(self):
        validator = number([double("must be a double")])
        self.assertTrue(validator(1.5).validate)
        result = validator(1)
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "must be a double")


if __name__ == "__main__":
    unittest.main()
