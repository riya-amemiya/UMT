import unittest

from src.validate.object.core import ObjectReturnType, object_validator


class TestObjectValidatorPropertyMessage(unittest.TestCase):
    def test_calls_each_property_validator_once_with_the_property_value(self):
        calls: list[object] = []

        def tracking_validator(value: object) -> ObjectReturnType:
            calls.append(value)
            ok = isinstance(value, str)
            return ObjectReturnType(
                validate=ok,
                message="" if ok else f"not a string: {value!r}",
                type=value,
            )

        validator = object_validator(
            {"first": tracking_validator, "second": tracking_validator}
        )
        result = validator({"first": 1, "second": "ok"})
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "not a string: 1")
        self.assertEqual(calls, [1])

    def test_propagates_the_message_of_the_failing_property_result(self):
        def always_failing(value: object) -> ObjectReturnType:
            return ObjectReturnType(
                validate=False,
                message="property" if value == 1 else "whole object",
                type=value,
            )

        result = object_validator({"a": always_failing})({"a": 1})
        self.assertEqual(result.message, "property")

    def test_ignores_shape_mutation_after_creation(self):
        def string_validator(value: object) -> ObjectReturnType:
            return ObjectReturnType(
                validate=isinstance(value, str), message="", type=value
            )

        shape = {"name": string_validator}
        validator = object_validator(shape)
        shape["age"] = string_validator
        self.assertTrue(validator({"name": "John"}).validate)


if __name__ == "__main__":
    unittest.main()
