import unittest

from src.validate import length_between, string


class TestLengthBetween(unittest.TestCase):
    def test_accepts_lengths_inside_the_range_including_bounds(self):
        validator = string([length_between(2, 4)])
        for value in ("ab", "abc", "abcd"):
            self.assertTrue(validator(value).validate)

    def test_rejects_lengths_outside_the_range_with_one_message(self):
        validator = string([length_between(2, 4, "length must be 2 to 4")])
        for value in ("a", "abcde"):
            result = validator(value)
            self.assertFalse(result.validate)
            self.assertEqual(result.message, "length must be 2 to 4")

    def test_defaults_the_message_to_empty_string(self):
        validator = string([length_between(2, 4)])
        self.assertFalse(validator("").validate)
        self.assertEqual(validator("").message, "")

    def test_measures_length_in_utf16_code_units(self):
        rule = length_between(2, 2)
        self.assertTrue(rule.validate("\U0001f600"))
        self.assertFalse(rule.validate("\u3042"))

    def test_rejects_every_value_when_min_is_greater_than_max(self):
        self.assertFalse(length_between(4, 2).validate("abc"))


if __name__ == "__main__":
    unittest.main()
