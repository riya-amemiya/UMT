import unittest

from src.validate import number, string
from src.validate.object import (
    IntersectionReturnType,
    intersection,
    object_validator,
)


class TestIntersection(unittest.TestCase):
    def test_passes_when_every_validator_passes(self):
        validator = intersection(
            object_validator({"name": string()}),
            object_validator({"age": number()}),
        )
        self.assertTrue(validator({"name": "John", "age": 30}).validate)

    def test_fails_when_any_validator_fails(self):
        validator = intersection(
            object_validator({"name": string()}),
            object_validator({"age": number()}),
        )
        self.assertFalse(validator({"name": "John"}).validate)
        self.assertFalse(validator({"age": 30}).validate)

    def test_returns_empty_message_on_success(self):
        validator = intersection(string([], "not a string"))
        self.assertEqual(validator("abc").message, "")

    def test_returns_the_first_failing_message(self):
        validator = intersection(
            object_validator({"name": string([], "bad name")}),
            object_validator({"age": number([], "bad age")}),
        )
        self.assertEqual(validator({}).message, "bad name")
        self.assertEqual(validator({"name": "John"}).message, "bad age")

    def test_stops_at_the_first_failing_validator(self):
        calls: list[str] = []

        def first(value: object) -> IntersectionReturnType:
            calls.append("first")
            return IntersectionReturnType(validate=False, message="first", type=value)

        def second(value: object) -> IntersectionReturnType:
            calls.append("second")
            return IntersectionReturnType(validate=True, message="", type=value)

        self.assertFalse(intersection(first, second)(1).validate)
        self.assertEqual(calls, ["first"])

    def test_passes_without_validators(self):
        result = intersection()(1)
        self.assertTrue(result.validate)
        self.assertEqual(result.message, "")

    def test_carries_the_value_through_the_type_field(self):
        value = {"name": "John"}
        validator = intersection(object_validator({"name": string()}))
        self.assertIs(validator(value).type, value)
        self.assertEqual(validator("nope").type, "nope")

    def test_returns_intersection_return_type(self):
        self.assertIsInstance(intersection(string())("a"), IntersectionReturnType)


if __name__ == "__main__":
    unittest.main()
