import unittest

from src.validate import StringReturnType, string
from src.validate.object import NullReturn, nullable, object_validator


class TestNullable(unittest.TestCase):
    def test_passes_for_none(self):
        result = nullable(string())(None)
        self.assertTrue(result.validate)
        self.assertEqual(result.message, "")
        assert isinstance(result, NullReturn)
        self.assertEqual(result.type, "null")

    def test_delegates_present_values_to_the_wrapped_validator(self):
        validator = nullable(string([], "must be a string"))
        valid = validator("abc")
        self.assertTrue(valid.validate)
        assert isinstance(valid, StringReturnType)
        self.assertEqual(valid.type, "string")
        invalid = validator(1)
        self.assertFalse(invalid.validate)
        self.assertEqual(invalid.message, "must be a string")

    def test_makes_an_object_property_nullable(self):
        validator = object_validator({"name": nullable(string())})
        self.assertTrue(validator({"name": None}).validate)
        self.assertTrue(validator({"name": "John"}).validate)
        self.assertFalse(validator({"name": 1}).validate)


if __name__ == "__main__":
    unittest.main()
