import unittest

from src.validate import even, number


class TestEven(unittest.TestCase):
    def test_returns_true_for_even_numbers(self):
        rule = even()
        for value in (2, 4, 100, 1000, 0, -2, -4, 4.0):
            self.assertTrue(rule.validate(value))

    def test_returns_false_for_odd_numbers(self):
        rule = even()
        for value in (1, 3, 99, 999, -1, -3):
            self.assertFalse(rule.validate(value))

    def test_returns_false_for_decimal_numbers(self):
        rule = even()
        for value in (2.5, 4.7, 1.3):
            self.assertFalse(rule.validate(value))

    def test_returns_false_for_non_finite_numbers(self):
        rule = even()
        self.assertFalse(rule.validate(float("nan")))
        self.assertFalse(rule.validate(float("inf")))

    def test_carries_the_custom_message(self):
        self.assertEqual(even("must be even").message, "must be even")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(even().message, "")

    def test_composes_with_number(self):
        validator = number([even("must be even")])
        self.assertTrue(validator(2).validate)
        self.assertEqual(validator(3).message, "must be even")


if __name__ == "__main__":
    unittest.main()
