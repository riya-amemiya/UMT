import unittest

from src.validate import NumberRule, number


def max_value(limit: float, message: str = "") -> NumberRule:
    return NumberRule(validate=lambda value: value <= limit, message=message)


def min_value(limit: float, message: str = "") -> NumberRule:
    return NumberRule(validate=lambda value: value >= limit, message=message)


class TestNumber(unittest.TestCase):
    def test_validates_number_with_no_additional_options(self):
        validator = number()
        self.assertTrue(validator(5).validate)
        self.assertTrue(validator(1.5).validate)
        self.assertTrue(validator(-3).validate)
        self.assertFalse(validator("5").validate)

    def test_rejects_non_number_values(self):
        validator = number()
        self.assertFalse(validator(None).validate)
        self.assertFalse(validator(True).validate)
        self.assertFalse(validator([]).validate)

    def test_validates_number_with_max_value_option(self):
        validator = number([max_value(10)])
        self.assertTrue(validator(5).validate)
        self.assertFalse(validator(11).validate)

    def test_validates_number_with_min_value_option(self):
        validator = number([min_value(3)])
        self.assertTrue(validator(5).validate)
        self.assertFalse(validator(2).validate)

    def test_validates_number_with_both_min_and_max_value_options(self):
        validator = number([min_value(3), max_value(10)])
        self.assertTrue(validator(5).validate)
        self.assertFalse(validator(2).validate)
        self.assertFalse(validator(11).validate)

    def test_returns_custom_message_on_validation_failure(self):
        custom_message = "Value must be between 3 and 10"
        validator = number([min_value(3, custom_message), max_value(10)])
        result = validator(2)
        self.assertFalse(result.validate)
        self.assertEqual(result.message, custom_message)

    def test_returns_configured_message_on_type_failure(self):
        validator = number([], "must be a number")
        self.assertEqual(validator("1").message, "must be a number")

    def test_falls_back_to_empty_message_when_rule_has_no_message(self):
        validator = number([min_value(3)])
        self.assertEqual(validator(2).message, "")

    def test_type_tag(self):
        validator = number()
        self.assertEqual(validator(1).type, "number")
        self.assertEqual(validator("1").type, "number")

    def test_docstring_example(self):
        validator = number()
        self.assertTrue(validator(5).validate)
        self.assertFalse(validator("5").validate)
        at_most_ten = NumberRule(
            validate=lambda value: value <= 10,
            message="must be at most 10",
        )
        self.assertEqual(number([at_most_ten])(11).message, "must be at most 10")


if __name__ == "__main__":
    unittest.main()
