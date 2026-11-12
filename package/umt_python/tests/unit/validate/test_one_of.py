import unittest

from src.validate import OneOfReturnType, one_of


class TestOneOf(unittest.TestCase):
    def test_accepts_allowed_values(self):
        validator = one_of(["standard", "squat", "flask"])
        self.assertTrue(validator("standard").validate)
        self.assertTrue(validator("flask").validate)

    def test_rejects_other_values(self):
        validator = one_of(["standard", "squat"])
        self.assertFalse(validator("round").validate)
        self.assertFalse(validator("Standard").validate)
        self.assertFalse(validator("").validate)

    def test_rejects_non_string_values(self):
        validator = one_of(["1"])
        self.assertFalse(validator(1).validate)
        self.assertFalse(validator(None).validate)
        self.assertFalse(validator(["1"]).validate)

    def test_rejects_everything_for_an_empty_list(self):
        self.assertFalse(one_of([])("a").validate)

    def test_returns_custom_message_on_failure(self):
        validator = one_of(["a", "b"], "must be a or b")
        self.assertEqual(validator("c").message, "must be a or b")

    def test_returns_empty_message_on_success(self):
        validator = one_of(["a", "b"], "must be a or b")
        self.assertEqual(validator("a").message, "")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(one_of(["a"])("b").message, "")

    def test_carries_the_value_through_the_type_field(self):
        validator = one_of(["a"])
        self.assertEqual(validator("a").type, "a")
        self.assertEqual(validator("b").type, "b")

    def test_returns_one_of_return_type(self):
        self.assertIsInstance(one_of(["a"])("a"), OneOfReturnType)


if __name__ == "__main__":
    unittest.main()
