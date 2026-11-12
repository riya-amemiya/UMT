import unittest

from src.validate import integer, number


class TestInteger(unittest.TestCase):
    def test_returns_true_for_integers(self):
        rule = integer()
        for value in (0, 1, -1, 1.0, 9007199254740991, 1e21, 10**30):
            self.assertTrue(rule.validate(value))

    def test_returns_false_for_non_integers(self):
        rule = integer()
        for value in (1.5, -0.1, float("nan"), float("inf"), float("-inf")):
            self.assertFalse(rule.validate(value))

    def test_carries_the_custom_message(self):
        self.assertEqual(integer("must be an integer").message, "must be an integer")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(integer().message, "")

    def test_composes_with_number(self):
        validator = number([integer("must be an integer")])
        self.assertTrue(validator(3).validate)
        result = validator(3.5)
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "must be an integer")
        self.assertFalse(validator(True).validate)


if __name__ == "__main__":
    unittest.main()
