import unittest

from src.validate import number, string
from src.validate.object import object_validator, omit_keys


class TestOmitKeys(unittest.TestCase):
    def setUp(self):
        self.user = object_validator(
            {"name": string(), "age": number(), "email": string()}
        )

    def test_drops_the_omitted_keys(self):
        omitted = omit_keys(self.user, ["age"])
        self.assertEqual(list(omitted.shape), ["name", "email"])

    def test_validates_only_the_remaining_keys(self):
        omitted = omit_keys(self.user, ["age", "email"])
        self.assertTrue(omitted({"name": "John"}).validate)
        self.assertTrue(omitted({"name": "John", "age": "thirty"}).validate)
        self.assertFalse(omitted({"name": 1}).validate)

    def test_reuses_the_source_validators(self):
        omitted = omit_keys(self.user, ["age"])
        self.assertIs(omitted.shape["name"], self.user.shape["name"])

    def test_leaves_the_source_validator_untouched(self):
        omit_keys(self.user, ["name"])
        self.assertEqual(list(self.user.shape), ["name", "age", "email"])

    def test_ignores_unknown_keys(self):
        omitted = omit_keys(self.user, ["missing"])
        self.assertEqual(list(omitted.shape), ["name", "age", "email"])

    def test_keeps_everything_for_an_empty_key_list(self):
        omitted = omit_keys(self.user, [])
        self.assertEqual(list(omitted.shape), ["name", "age", "email"])

    def test_uses_the_custom_message_for_non_dictionary_values(self):
        omitted = omit_keys(self.user, ["age"], "must be an object")
        self.assertEqual(omitted("nope").message, "must be an object")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(omit_keys(self.user, ["age"])("nope").message, "")


if __name__ == "__main__":
    unittest.main()
