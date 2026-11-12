import unittest

from src.validate import number, string
from src.validate.object import UnionReturnType, union


class TestUnion(unittest.TestCase):
    def test_passes_when_any_validator_passes(self):
        validator = union(string(), number())
        self.assertTrue(validator("abc").validate)
        self.assertTrue(validator(1).validate)

    def test_fails_when_no_validator_passes(self):
        validator = union(string(), number())
        self.assertFalse(validator(None).validate)
        self.assertFalse(validator([]).validate)

    def test_returns_empty_message_on_success(self):
        validator = union(string([], "not a string"), number([], "not a number"))
        self.assertEqual(validator(1).message, "")

    def test_returns_the_last_message_on_failure(self):
        validator = union(string([], "not a string"), number([], "not a number"))
        self.assertEqual(validator(None).message, "not a number")

    def test_stops_at_the_first_passing_validator(self):
        calls: list[str] = []

        def first(value: object) -> UnionReturnType:
            calls.append("first")
            return UnionReturnType(validate=True, message="", type=value)

        def second(value: object) -> UnionReturnType:
            calls.append("second")
            return UnionReturnType(validate=True, message="", type=value)

        self.assertTrue(union(first, second)(1).validate)
        self.assertEqual(calls, ["first"])

    def test_fails_with_empty_message_without_validators(self):
        result = union()(1)
        self.assertFalse(result.validate)
        self.assertEqual(result.message, "")

    def test_carries_the_value_through_the_type_field(self):
        value = {"a": 1}
        validator = union(string(), number())
        self.assertIs(validator(value).type, value)
        self.assertEqual(validator("abc").type, "abc")

    def test_returns_union_return_type(self):
        self.assertIsInstance(union(string())("a"), UnionReturnType)


if __name__ == "__main__":
    unittest.main()
