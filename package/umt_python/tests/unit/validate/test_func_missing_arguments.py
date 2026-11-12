import unittest

from src.validate import FunctionReturnType, FunctionSchema, func


def number_validator(value: object) -> FunctionReturnType:
    valid = isinstance(value, (int, float)) and not isinstance(value, bool)
    return FunctionReturnType(validate=valid, message="", type=value)


def optional_number_validator(value: object) -> FunctionReturnType:
    valid = value is None or isinstance(value, int)
    return FunctionReturnType(validate=valid, message="", type=value)


class TestFuncMissingArguments(unittest.TestCase):
    def test_missing_argument_is_validated_as_none(self):
        validator = func(FunctionSchema(input=[number_validator, number_validator]))
        wrapped = validator.implement(lambda *arguments: len(arguments))
        with self.assertRaisesRegex(
            TypeError, "function input at index 1 failed validation"
        ):
            wrapped(1)

    def test_missing_argument_accepted_by_its_validator(self):
        validator = func(
            FunctionSchema(input=[number_validator, optional_number_validator])
        )
        wrapped = validator.implement(lambda *arguments: len(arguments))
        self.assertEqual(wrapped(1), 1)
        self.assertEqual(wrapped(1, 2), 2)

    def test_implement_uses_an_empty_custom_message(self):
        validator = func(None, "")
        with self.assertRaises(TypeError) as context:
            validator.implement("not a function")  # type: ignore[arg-type]
        self.assertEqual(str(context.exception), "")

    def test_implement_uses_the_custom_message(self):
        validator = func(None, "must be a function")
        with self.assertRaisesRegex(TypeError, "must be a function"):
            validator.implement(1)  # type: ignore[arg-type]

    def test_implement_falls_back_to_the_default_message(self):
        with self.assertRaisesRegex(TypeError, "value is not a function"):
            func().implement(1)  # type: ignore[arg-type]


if __name__ == "__main__":
    unittest.main()
