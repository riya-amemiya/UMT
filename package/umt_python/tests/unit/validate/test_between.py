import unittest

from src.validate import between, number


class TestBetween(unittest.TestCase):
    def test_accepts_values_inside_the_range_including_bounds(self):
        rule = between(1, 10)
        for value in (1, 5.5, 10):
            self.assertTrue(rule.validate(value))

    def test_rejects_values_below_the_minimum(self):
        rule = between(1, 10)
        for value in (0.999, -1):
            self.assertFalse(rule.validate(value))

    def test_rejects_values_above_the_maximum(self):
        rule = between(1, 10)
        for value in (10.001, 100):
            self.assertFalse(rule.validate(value))

    def test_rejects_nan(self):
        self.assertFalse(between(1, 10).validate(float("nan")))

    def test_accepts_only_the_bound_when_min_equals_max(self):
        rule = between(3, 3)
        self.assertTrue(rule.validate(3))
        self.assertFalse(rule.validate(2))
        self.assertFalse(rule.validate(4))

    def test_rejects_every_value_when_min_is_greater_than_max(self):
        rule = between(10, 1)
        for value in (1, 5, 10):
            self.assertFalse(rule.validate(value))

    def test_reports_one_message_for_both_bounds(self):
        validator = number([between(1, 10, "must be between 1 and 10")])
        self.assertTrue(validator(5).validate)
        self.assertEqual(validator(0).message, "must be between 1 and 10")
        self.assertEqual(validator(11).message, "must be between 1 and 10")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(between(1, 10).message, "")


if __name__ == "__main__":
    unittest.main()
