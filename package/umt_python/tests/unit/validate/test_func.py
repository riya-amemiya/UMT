import unittest

from src.validate import FunctionReturnType, FunctionSchema, func


def number_validator(value: object) -> FunctionReturnType:
    valid = isinstance(value, (int, float)) and not isinstance(value, bool)
    return FunctionReturnType(validate=valid, message="", type=value)


def string_validator(value: object) -> FunctionReturnType:
    return FunctionReturnType(validate=isinstance(value, str), message="", type=value)


class TestFunc(unittest.TestCase):
    def test_accepts_function_values(self):
        validator = func()
        self.assertTrue(validator(lambda: 0).validate)

    def test_rejects_non_function_values(self):
        validator = func()
        self.assertFalse(validator("nope").validate)
        self.assertFalse(validator(42).validate)

    def test_returns_configured_message_on_failure(self):
        validator = func(None, "must be a function")
        self.assertEqual(validator("nope").message, "must be a function")

    def test_carries_value_through_type_field(self):
        validator = func()

        def handler():
            return 1

        self.assertIs(validator(handler).type, handler)

    def test_validates_input_arguments_via_implement(self):
        validator = func(
            FunctionSchema(
                input=[string_validator, number_validator],
                output=number_validator,
            )
        )
        wrapped = validator.implement(lambda name, age: age + len(name))
        self.assertEqual(wrapped("John", 30), 34)
        with self.assertRaises(TypeError):
            wrapped(1, 30)
        with self.assertRaises(TypeError):
            wrapped("John", "30")

    def test_validates_output_via_implement(self):
        validator = func(
            FunctionSchema(input=[number_validator], output=string_validator)
        )
        wrapped = validator.implement(lambda n: n)
        with self.assertRaises(TypeError):
            wrapped(1)

    def test_default_message_when_input_validation_fails(self):
        validator = func(
            FunctionSchema(input=[number_validator], output=number_validator)
        )
        wrapped = validator.implement(lambda n: n)
        with self.assertRaisesRegex(
            TypeError, "function input at index 0 failed validation"
        ):
            wrapped("nope")

    def test_default_message_when_output_validation_fails(self):
        validator = func(
            FunctionSchema(input=[number_validator], output=number_validator)
        )
        wrapped = validator.implement(lambda n: str(n))
        with self.assertRaisesRegex(TypeError, "function output failed validation"):
            wrapped(1)

    def test_implement_rejects_non_function_with_default_message(self):
        validator = func()
        with self.assertRaisesRegex(TypeError, "value is not a function"):
            validator.implement("not a function")  # type: ignore[arg-type]

    def test_implement_rejects_non_function_with_custom_message(self):
        validator = func(None, "must be callable")
        with self.assertRaisesRegex(TypeError, "must be callable"):
            validator.implement("not a function")  # type: ignore[arg-type]

    def test_skips_input_checks_with_no_input_schema(self):
        validator = func(FunctionSchema(output=number_validator))
        wrapped = validator.implement(lambda: 1)
        self.assertEqual(wrapped(), 1)

    def test_skips_output_checks_with_no_output_schema(self):
        validator = func(FunctionSchema(input=[number_validator]))
        wrapped = validator.implement(lambda n: n)
        self.assertEqual(wrapped(2), 2)

    def test_returns_value_untouched_with_no_schema(self):
        validator = func()
        wrapped = validator.implement(lambda value: value * 2)
        self.assertEqual(wrapped(3), 6)

    def test_wrapper_does_not_change_call_results(self):
        validator = func(
            FunctionSchema(input=[number_validator], output=number_validator)
        )
        wrapped = validator.implement(lambda n: n * 2)
        self.assertEqual(wrapped(2), 4)
        self.assertEqual(wrapped(3), 6)

    def test_docstring_example(self):
        validator = func()
        self.assertTrue(validator(lambda: 0).validate)
        self.assertFalse(validator("nope").validate)


if __name__ == "__main__":
    unittest.main()
