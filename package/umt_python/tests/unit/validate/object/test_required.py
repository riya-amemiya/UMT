import unittest

from src.validate import number, string
from src.validate.object import object_validator, optional, partial, required


class TestRequired(unittest.TestCase):
    def test_removes_optional_wrappers(self):
        user = object_validator({"name": optional(string()), "age": optional(number())})
        self.assertTrue(user({}).validate)
        validator = required(user)
        self.assertFalse(validator({}).validate)
        self.assertFalse(validator({"name": "John"}).validate)
        self.assertTrue(validator({"name": "John", "age": 30}).validate)

    def test_restores_the_inner_validator(self):
        inner = string()
        user = object_validator({"name": optional(inner)})
        self.assertIs(required(user).shape["name"], inner)

    def test_keeps_validators_that_are_not_optional(self):
        inner = string()
        user = object_validator({"name": inner})
        self.assertIs(required(user).shape["name"], inner)

    def test_undoes_partial(self):
        user = object_validator({"name": string([], "bad name")})
        validator = required(partial(user))
        result = validator({})
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "bad name")

    def test_leaves_the_source_validator_untouched(self):
        user = object_validator({"name": optional(string())})
        required(user)
        self.assertTrue(user({}).validate)

    def test_uses_the_custom_message_for_non_dictionary_values(self):
        user = object_validator({"name": optional(string())})
        validator = required(user, "must be an object")
        self.assertEqual(validator("nope").message, "must be an object")

    def test_defaults_the_message_to_empty_string(self):
        user = object_validator({"name": optional(string())})
        self.assertEqual(required(user)("nope").message, "")


if __name__ == "__main__":
    unittest.main()
