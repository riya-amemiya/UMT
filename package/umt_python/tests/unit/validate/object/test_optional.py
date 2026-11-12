import unittest

from src.validate import number, string
from src.validate.object import (
    OptionalValidator,
    UndefinedReturn,
    object_validator,
    optional,
)


class TestOptional(unittest.TestCase):
    def test_passes_for_none(self):
        result = optional(string())(None)
        self.assertTrue(result.validate)
        self.assertEqual(result.message, "")
        assert isinstance(result, UndefinedReturn)
        self.assertEqual(result.type, "undefined")

    def test_passes_when_called_without_a_value(self):
        self.assertTrue(optional(string())().validate)

    def test_delegates_present_values_to_the_wrapped_validator(self):
        validator = optional(string([], "must be a string"))
        self.assertTrue(validator("abc").validate)
        result = validator(1)
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "must be a string")

    def test_exposes_the_wrapped_validator(self):
        inner = number()
        validator = optional(inner)
        self.assertIsInstance(validator, OptionalValidator)
        self.assertIs(validator.inner, inner)
        self.assertIs(validator.is_optional, True)

    def test_makes_an_object_property_optional(self):
        validator = object_validator({"name": string(), "age": optional(number())})
        self.assertTrue(validator({"name": "John"}).validate)
        self.assertTrue(validator({"name": "John", "age": 30}).validate)
        self.assertFalse(validator({"name": "John", "age": "30"}).validate)


if __name__ == "__main__":
    unittest.main()
