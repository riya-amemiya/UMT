import unittest

from src.validate import number, string
from src.validate.object import object_validator, pick_keys


class TestPickKeys(unittest.TestCase):
    def setUp(self):
        self.user = object_validator(
            {"name": string(), "age": number(), "email": string()}
        )

    def test_keeps_only_the_picked_keys(self):
        picked = pick_keys(self.user, ["name", "email"])
        self.assertEqual(list(picked.shape), ["name", "email"])

    def test_validates_only_the_picked_keys(self):
        picked = pick_keys(self.user, ["name"])
        self.assertTrue(picked({"name": "John"}).validate)
        self.assertTrue(picked({"name": "John", "age": "thirty"}).validate)
        self.assertFalse(picked({"name": 1}).validate)

    def test_reuses_the_source_validators(self):
        picked = pick_keys(self.user, ["age"])
        self.assertIs(picked.shape["age"], self.user.shape["age"])

    def test_leaves_the_source_validator_untouched(self):
        pick_keys(self.user, ["name"])
        self.assertEqual(list(self.user.shape), ["name", "age", "email"])

    def test_picks_nothing_for_an_empty_key_list(self):
        picked = pick_keys(self.user, [])
        self.assertEqual(list(picked.shape), [])
        self.assertTrue(picked({}).validate)

    def test_uses_the_custom_message_for_non_dictionary_values(self):
        picked = pick_keys(self.user, ["name"], "must be an object")
        self.assertEqual(picked("nope").message, "must be an object")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(pick_keys(self.user, ["name"])("nope").message, "")

    def test_raises_for_unknown_keys(self):
        with self.assertRaises(KeyError):
            pick_keys(self.user, ["missing"])


if __name__ == "__main__":
    unittest.main()
