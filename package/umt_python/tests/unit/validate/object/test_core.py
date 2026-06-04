import unittest
from collections.abc import Callable

from src.validate.object.core import ObjectReturnType, object_validator


def _string_validator(value: object) -> ObjectReturnType:
    ok = isinstance(value, str)
    return ObjectReturnType(
        validate=ok,
        message="" if ok else "not a string",
        type=value,
    )


def _number_validator(
    message: str = "",
) -> Callable[[object], ObjectReturnType]:
    def validator(value: object) -> ObjectReturnType:
        ok = isinstance(value, (int, float)) and not isinstance(value, bool)
        return ObjectReturnType(
            validate=ok,
            message="" if ok else message,
            type=value,
        )

    return validator


class TestObjectValidator(unittest.TestCase):
    def test_validates_an_empty_shape_against_any_dictionary(self):
        validator = object_validator()
        result = validator({"name": "John Doe", "age": "thirty"})
        self.assertTrue(result.validate)

    def test_validates_an_object_with_matching_property_types(self):
        validator = object_validator(
            {"name": _string_validator, "age": _number_validator()}
        )
        result = validator({"name": "John Doe", "age": 30})
        self.assertTrue(result.validate)
        self.assertEqual(result.message, "")

    def test_fails_when_a_property_does_not_match(self):
        validator = object_validator(
            {"name": _string_validator, "age": _number_validator()}
        )
        self.assertFalse(validator({"name": "John Doe", "age": "thirty"}).validate)

    def test_propagates_a_custom_property_message(self):
        custom_message = "Invalid object structure"
        validator = object_validator(
            {"name": _string_validator, "age": _number_validator(custom_message)}
        )
        result = validator({"name": "John Doe", "age": "thirty"})
        self.assertFalse(result.validate)
        self.assertEqual(result.message, custom_message)

    def test_returns_the_custom_message_when_value_is_not_a_dictionary(self):
        custom_message = "Invalid object structure"
        validator = object_validator(
            {"name": _string_validator, "age": _number_validator()},
            custom_message,
        )
        result = validator("John Doe")
        self.assertFalse(result.validate)
        self.assertEqual(result.message, custom_message)

    def test_defaults_the_dictionary_message_to_empty_string(self):
        validator = object_validator(
            {"name": _string_validator, "age": _number_validator()}
        )
        result = validator("John Doe")
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "")

    def test_rejects_a_list(self):
        validator = object_validator()
        self.assertFalse(validator([]).validate)

    def test_treats_a_missing_property_as_none(self):
        validator = object_validator({"name": _string_validator})
        result = validator({})
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "not a string")

    def test_preserves_the_original_value_in_the_type_field(self):
        data = {"name": "John Doe"}
        validator = object_validator({"name": _string_validator})
        self.assertIs(validator(data).type, data)

    def test_stops_at_the_first_failing_property(self):
        calls: list[object] = []

        def tracking_validator(value: object) -> ObjectReturnType:
            calls.append(value)
            ok = isinstance(value, str)
            return ObjectReturnType(
                validate=ok,
                message="" if ok else "not a string",
                type=value,
            )

        validator = object_validator(
            {"first": tracking_validator, "second": tracking_validator}
        )
        result = validator({"first": 1, "second": "ok"})
        self.assertFalse(result.validate)
        self.assertEqual(calls, [1, {"first": 1, "second": "ok"}])

    def test_exposes_the_shape_attribute(self):
        shape = {"name": _string_validator}
        validator = object_validator(shape)
        self.assertIs(validator.shape, shape)

    def test_docstring_example(self):
        validator = object_validator({"name": _string_validator})
        self.assertTrue(validator({"name": "John"}).validate)
        self.assertFalse(validator({"name": 42}).validate)
        self.assertFalse(validator("not a dict").validate)
        self.assertTrue(object_validator()({"anything": 1}).validate)


if __name__ == "__main__":
    unittest.main()
