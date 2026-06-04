import unittest

from src.validate.map import MapReturnType, map_validator


def _string_validator(value: object) -> MapReturnType:
    ok = isinstance(value, str)
    return MapReturnType(
        validate=ok,
        message="" if ok else "not a string",
        type=value,
    )


def _number_validator(value: object) -> MapReturnType:
    ok = isinstance(value, (int, float)) and not isinstance(value, bool)
    return MapReturnType(
        validate=ok,
        message="" if ok else "not a number",
        type=value,
    )


class TestMap(unittest.TestCase):
    def test_accepts_a_map_without_entry_validators(self):
        validator = map_validator()
        self.assertTrue(validator({}).validate)
        self.assertTrue(validator({"a": 1}).validate)

    def test_rejects_non_map_values(self):
        validator = map_validator()
        self.assertFalse(validator([]).validate)
        self.assertFalse(validator(None).validate)
        self.assertFalse(validator("nope").validate)

    def test_validates_keys_with_the_key_validator(self):
        validator = map_validator(_string_validator, _number_validator)
        self.assertTrue(validator({"a": 1, "b": 2}).validate)
        self.assertFalse(validator({1: 1}).validate)

    def test_validates_values_with_the_value_validator(self):
        validator = map_validator(_string_validator, _number_validator)
        self.assertFalse(validator({"a": "not a number"}).validate)

    def test_returns_the_configured_message_when_not_a_map(self):
        validator = map_validator(_string_validator, _number_validator, "must be a Map")
        self.assertEqual(validator([]).message, "must be a Map")

    def test_defaults_the_message_to_empty_string(self):
        validator = map_validator()
        result = validator(None)
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "")

    def test_propagates_the_failing_entry_message(self):
        validator = map_validator(_string_validator, _number_validator)
        result = validator({"a": "x"})
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "not a number")

    def test_propagates_the_failing_key_message(self):
        validator = map_validator(_string_validator, _number_validator)
        result = validator({1: 1})
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "not a string")

    def test_short_circuits_keys_before_values(self):
        validator = map_validator(_string_validator, _number_validator)
        result = validator({1: "also wrong"})
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "not a string")

    def test_preserves_the_original_map_in_the_type_field(self):
        data = {"a": 1}
        validator = map_validator(_string_validator, _number_validator)
        self.assertIs(validator(data).type, data)

    def test_stops_at_the_first_failing_entry(self):
        calls: list[object] = []

        def tracking_validator(value: object) -> MapReturnType:
            calls.append(value)
            ok = isinstance(value, int) and value >= 0
            return MapReturnType(
                validate=ok,
                message="" if ok else "negative",
                type=value,
            )

        validator = map_validator(value_validator=tracking_validator)
        result = validator({"a": 1, "b": -1, "c": -2})
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "negative")
        self.assertEqual(calls, [1, -1])

    def test_docstring_example(self):
        validator = map_validator(_string_validator, _number_validator)
        self.assertTrue(validator({"a": 1, "b": 2}).validate)
        self.assertFalse(validator({"a": "x"}).validate)
        self.assertFalse(validator("not a map").validate)


if __name__ == "__main__":
    unittest.main()
