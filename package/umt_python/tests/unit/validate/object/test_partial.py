import unittest

from src.validate import number, string
from src.validate.object import (
    OptionalValidator,
    object_validator,
    optional,
    partial,
)


class TestPartial(unittest.TestCase):
    def test_makes_every_key_optional(self):
        user = object_validator({"name": string(), "age": number()})
        self.assertFalse(user({}).validate)
        validator = partial(user)
        self.assertTrue(validator({}).validate)
        self.assertTrue(validator({"name": "John"}).validate)
        self.assertTrue(validator({"name": "John", "age": 30}).validate)

    def test_still_validates_present_values(self):
        validator = partial(object_validator({"name": string([], "bad name")}))
        result = validator({"name": 1})
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "bad name")

    def test_wraps_validators_with_optional(self):
        user = object_validator({"name": string()})
        wrapped = partial(user).shape["name"]
        assert isinstance(wrapped, OptionalValidator)
        self.assertIs(wrapped.inner, user.shape["name"])

    def test_keeps_already_optional_validators(self):
        already_optional = optional(string())
        user = object_validator({"name": already_optional})
        self.assertIs(partial(user).shape["name"], already_optional)

    def test_leaves_the_source_validator_untouched(self):
        user = object_validator({"name": string()})
        partial(user)
        self.assertFalse(user({}).validate)

    def test_uses_the_custom_message_for_non_dictionary_values(self):
        validator = partial(object_validator({"name": string()}), "must be an object")
        self.assertEqual(validator("nope").message, "must be an object")

    def test_defaults_the_message_to_empty_string(self):
        validator = partial(object_validator({"name": string()}))
        self.assertEqual(validator("nope").message, "")


if __name__ == "__main__":
    unittest.main()
