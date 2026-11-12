import unittest

from src.validate import number_string, string


class TestNumberString(unittest.TestCase):
    def test_returns_true_for_number_strings(self):
        rule = number_string()
        for value in ("123", "-1.5", "0", " 42 ", "0x10", "1e3", ""):
            self.assertTrue(rule.validate(value))

    def test_returns_false_for_non_number_strings(self):
        rule = number_string()
        for value in ("abc", "12abc", "NaN", "Infinity", "1_000"):
            self.assertFalse(rule.validate(value))

    def test_carries_the_custom_message(self):
        self.assertEqual(number_string("not a number").message, "not a number")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(number_string().message, "")

    def test_composes_with_string(self):
        validator = string([number_string("not a number")])
        self.assertTrue(validator("123").validate)
        self.assertEqual(validator("abc").message, "not a number")


if __name__ == "__main__":
    unittest.main()
