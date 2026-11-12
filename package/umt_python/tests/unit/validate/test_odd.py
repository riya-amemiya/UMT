import unittest

from src.validate import number, odd


class TestOdd(unittest.TestCase):
    def test_returns_true_for_odd_numbers(self):
        rule = odd()
        for value in (1, 3, 99, 999, -1, -3, 3.0):
            self.assertTrue(rule.validate(value))

    def test_returns_false_for_even_numbers(self):
        rule = odd()
        for value in (2, 4, 100, 0, -2):
            self.assertFalse(rule.validate(value))

    def test_returns_false_for_decimal_numbers(self):
        rule = odd()
        for value in (1.5, 3.7, 2.3):
            self.assertFalse(rule.validate(value))

    def test_returns_true_for_non_finite_numbers(self):
        rule = odd()
        self.assertTrue(rule.validate(float("nan")))
        self.assertTrue(rule.validate(float("inf")))

    def test_carries_the_custom_message(self):
        self.assertEqual(odd("must be odd").message, "must be odd")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(odd().message, "")

    def test_composes_with_number(self):
        validator = number([odd("must be odd")])
        self.assertTrue(validator(3).validate)
        self.assertEqual(validator(2).message, "must be odd")


if __name__ == "__main__":
    unittest.main()
