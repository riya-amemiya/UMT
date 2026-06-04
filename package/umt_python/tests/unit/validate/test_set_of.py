import unittest

from src.validate.set_of import SetOfReturnType, set_of


def _string_validator(value: object) -> SetOfReturnType:
    ok = isinstance(value, str)
    return SetOfReturnType(
        validate=ok,
        message="" if ok else "not a string",
        type=value,
    )


class TestSetOf(unittest.TestCase):
    def test_accepts_a_set_without_item_validator(self):
        validator = set_of()
        self.assertTrue(validator(set()).validate)
        self.assertTrue(validator({1, 2}).validate)

    def test_rejects_non_set_values(self):
        validator = set_of()
        self.assertFalse(validator([1, 2]).validate)
        self.assertFalse(validator({}).validate)
        self.assertFalse(validator(None).validate)

    def test_validates_items_with_the_item_validator(self):
        validator = set_of(_string_validator)
        self.assertTrue(validator({"a", "b"}).validate)
        self.assertFalse(validator({1}).validate)

    def test_returns_the_configured_message_when_not_a_set(self):
        validator = set_of(_string_validator, "must be a Set")
        self.assertEqual(validator([]).message, "must be a Set")

    def test_propagates_the_failing_item_message(self):
        validator = set_of(_string_validator)
        result = validator({1})
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "not a string")

    def test_defaults_the_message_to_empty_string(self):
        validator = set_of(_string_validator)
        result = validator(None)
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "")

    def test_preserves_the_original_set_in_the_type_field(self):
        data = {"a", "b"}
        validator = set_of(_string_validator)
        self.assertIs(validator(data).type, data)

    def test_stops_at_the_first_failing_element(self):
        calls: list[object] = []

        def tracking_validator(value: object) -> SetOfReturnType:
            calls.append(value)
            ok = isinstance(value, str)
            return SetOfReturnType(
                validate=ok,
                message="" if ok else "not a string",
                type=value,
            )

        validator = set_of(tracking_validator)
        result = validator({1})
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "not a string")
        self.assertEqual(calls, [1])

    def test_docstring_example(self):
        validator = set_of(_string_validator)
        self.assertTrue(validator({"a", "b"}).validate)
        self.assertFalse(validator({1}).validate)
        self.assertFalse(set_of()([1, 2]).validate)
        self.assertTrue(set_of()(set()).validate)


if __name__ == "__main__":
    unittest.main()
