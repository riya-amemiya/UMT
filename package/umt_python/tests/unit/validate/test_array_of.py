import unittest

from src.validate.array_of import ArrayOfReturnType, array_of


def _number_validator(value: object) -> ArrayOfReturnType:
    ok = isinstance(value, (int, float)) and not isinstance(value, bool)
    return ArrayOfReturnType(
        validate=ok,
        message="" if ok else "not a number",
        type=value,
    )


def _string_validator(value: object) -> ArrayOfReturnType:
    ok = isinstance(value, str)
    return ArrayOfReturnType(
        validate=ok,
        message="" if ok else "not a string",
        type=value,
    )


class TestArrayOf(unittest.TestCase):
    def test_validates_an_empty_array(self):
        validator = array_of(_string_validator)
        self.assertTrue(validator([]).validate)

    def test_validates_an_array_of_valid_elements(self):
        validator = array_of(_number_validator)
        result = validator([1, 2, 3])
        self.assertTrue(result.validate)
        self.assertEqual(result.message, "")

    def test_fails_when_an_element_does_not_match(self):
        validator = array_of(_number_validator)
        self.assertFalse(validator([1, "two", 3]).validate)

    def test_fails_when_the_value_is_not_an_array(self):
        validator = array_of(_string_validator, "not an array")
        none_result = validator(None)
        self.assertFalse(none_result.validate)
        self.assertEqual(none_result.message, "not an array")
        string_result = validator("not array")
        self.assertFalse(string_result.validate)
        self.assertEqual(string_result.message, "not an array")

    def test_defaults_the_message_to_empty_string(self):
        validator = array_of(_string_validator)
        result = validator(None)
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "")

    def test_propagates_the_element_validation_message(self):
        validator = array_of(_string_validator)
        result = validator([1])
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "not a string")

    def test_preserves_the_original_array_in_the_type_field(self):
        data = ["a", "b"]
        validator = array_of(_string_validator)
        self.assertIs(validator(data).type, data)

    def test_supports_nesting_array_of(self):
        validator = array_of(array_of(_number_validator))
        self.assertTrue(validator([[1, 2], [3, 4]]).validate)
        self.assertFalse(validator([[1, "2"]]).validate)

    def test_stops_at_the_first_failing_element(self):
        calls: list[object] = []

        def tracking_validator(value: object) -> ArrayOfReturnType:
            calls.append(value)
            ok = isinstance(value, int) and value >= 0
            return ArrayOfReturnType(
                validate=ok,
                message="" if ok else "negative",
                type=value,
            )

        validator = array_of(tracking_validator)
        result = validator([1, -1, -2])
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "negative")
        self.assertEqual(calls, [1, -1])

    def test_docstring_example(self):
        validator = array_of(_string_validator)
        self.assertTrue(validator(["a", "b"]).validate)
        self.assertFalse(validator(["a", 1]).validate)
        self.assertFalse(validator("not a list").validate)


if __name__ == "__main__":
    unittest.main()
