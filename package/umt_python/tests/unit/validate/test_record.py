import unittest

from src.validate import (
    RecordReturnType,
    array_of,
    min_length,
    number,
    object_validator,
    one_of,
    record,
    string,
)


class TestRecord(unittest.TestCase):
    def test_accepts_an_empty_dictionary(self):
        self.assertEqual(
            record(string(), number())({}),
            RecordReturnType(validate=True, message="", type={}),
        )

    def test_validates_every_key_and_value(self):
        self.assertTrue(record(string(), number())({"a": 1, "b": 2}).validate)

    def test_propagates_the_failing_value_message(self):
        validator = record(string(), number([], "value must be a number"))
        result = validator({"a": 1, "b": "x"})
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "value must be a number")

    def test_propagates_the_failing_key_message(self):
        validator = record(
            string([min_length(2, "key too short")]),
            number([], "value must be a number"),
        )
        result = validator({"ab": 1, "c": 2})
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "key too short")

    def test_checks_the_key_before_its_value(self):
        validator = record(
            string([min_length(2, "key too short")]),
            number([], "value must be a number"),
        )
        self.assertEqual(validator({"c": "x"}).message, "key too short")

    def test_rejects_non_string_keys_with_a_string_key_validator(self):
        validator = record(string([], "key must be a string"), number())
        self.assertEqual(validator({1: 1}).message, "key must be a string")

    def test_restricts_keys_with_one_of(self):
        validator = record(one_of(["en", "ja"], "unknown locale"), string())
        self.assertTrue(validator({"en": "Hello", "ja": "Konnichiwa"}).validate)
        self.assertEqual(
            validator({"en": "Hello", "fr": "Bonjour"}).message, "unknown locale"
        )

    def test_rejects_non_dictionary_values(self):
        validator = record(string(), number(), "must be a record")
        for value in (None, [1, 2], "a", {1, 2}):
            result = validator(value)
            self.assertFalse(result.validate)
            self.assertEqual(result.message, "must be a record")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(record(string(), number())(None).message, "")

    def test_composes_inside_object_validator_and_array_of(self):
        validator = object_validator({"scores": record(string(), array_of(number()))})
        self.assertTrue(validator({"scores": {"alice": [1, 2], "bob": []}}).validate)
        self.assertFalse(validator({"scores": {"alice": ["x"]}}).validate)


if __name__ == "__main__":
    unittest.main()
