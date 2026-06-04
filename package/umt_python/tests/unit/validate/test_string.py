import unittest

from src.validate import StringRule, string


def min_length(limit: int, message: str = "") -> StringRule:
    return StringRule(validate=lambda value: len(value) >= limit, message=message)


def max_length(limit: int, message: str = "") -> StringRule:
    return StringRule(validate=lambda value: len(value) <= limit, message=message)


class TestString(unittest.TestCase):
    def test_validates_string_with_no_additional_options(self):
        validator = string()
        self.assertTrue(validator("").validate)
        self.assertTrue(validator("abc").validate)
        self.assertFalse(validator(123).validate)

    def test_rejects_non_string_values(self):
        validator = string()
        self.assertFalse(validator(None).validate)
        self.assertFalse(validator(True).validate)
        self.assertFalse(validator([]).validate)

    def test_returns_empty_message_for_valid_string_with_message(self):
        validator = string([], "Must be string")
        self.assertEqual(validator("").message, "")
        self.assertTrue(validator("abc").validate)
        self.assertFalse(validator(123).validate)

    def test_can_specify_the_maximum_and_minimum_number_of_characters(self):
        validator = string([min_length(3), max_length(5)])
        self.assertFalse(validator("ab").validate)
        self.assertTrue(validator("abc").validate)
        self.assertTrue(validator("abcde").validate)
        self.assertFalse(validator("abcdef").validate)
        self.assertFalse(validator("abcdefg").validate)

    def test_returns_custom_message_on_validation_failure(self):
        custom_message = "Length must be between 3 and 5"
        validator = string([min_length(3, custom_message), max_length(5)])
        result = validator("ab")
        self.assertFalse(result.validate)
        self.assertEqual(result.message, custom_message)

    def test_returns_configured_message_on_type_failure(self):
        validator = string([], "must be a string")
        self.assertEqual(validator(1).message, "must be a string")

    def test_falls_back_to_empty_message_when_rule_has_no_message(self):
        validator = string([min_length(3)])
        self.assertEqual(validator("ab").message, "")

    def test_type_tag(self):
        validator = string()
        self.assertEqual(validator("a").type, "string")
        self.assertEqual(validator(1).type, "string")

    def test_docstring_example(self):
        validator = string()
        self.assertTrue(validator("abc").validate)
        self.assertFalse(validator(123).validate)
        at_most_five = StringRule(
            validate=lambda value: len(value) <= 5,
            message="must be at most 5 characters",
        )
        self.assertEqual(
            string([at_most_five])("abcdef").message,
            "must be at most 5 characters",
        )


if __name__ == "__main__":
    unittest.main()
