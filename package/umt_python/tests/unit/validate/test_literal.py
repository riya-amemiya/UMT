import unittest

from src.validate import (
    LiteralReturnType,
    array_of,
    literal,
    number,
    object_validator,
    string,
    union,
)


class TestLiteral(unittest.TestCase):
    def test_accepts_only_the_given_string(self):
        validator = literal("admin")
        self.assertEqual(
            validator("admin"),
            LiteralReturnType(validate=True, message="", type="admin"),
        )
        self.assertFalse(validator("user").validate)

    def test_accepts_only_the_given_number(self):
        validator = literal(42)
        self.assertTrue(validator(42).validate)
        self.assertTrue(validator(42.0).validate)
        self.assertFalse(validator(43).validate)
        self.assertFalse(validator("42").validate)

    def test_accepts_only_the_given_boolean(self):
        validator = literal(True)
        self.assertTrue(validator(True).validate)
        self.assertFalse(validator(False).validate)
        self.assertFalse(validator(1).validate)
        self.assertFalse(literal(1)(True).validate)

    def test_accepts_none(self):
        validator = literal(None)
        self.assertTrue(validator(None).validate)
        self.assertFalse(validator(0).validate)
        self.assertFalse(validator("").validate)

    def test_uses_strict_equality_for_nan_and_signed_zero(self):
        self.assertFalse(literal(float("nan"))(float("nan")).validate)
        self.assertTrue(literal(0)(-0.0).validate)

    def test_returns_the_custom_message_on_failure_only(self):
        validator = literal("a", "must be a")
        self.assertEqual(validator("b").message, "must be a")
        self.assertEqual(validator("a").message, "")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(literal("a")("b").message, "")

    def test_composes_inside_object_validator(self):
        validator = object_validator({"kind": literal("circle"), "radius": number()})
        self.assertTrue(validator({"kind": "circle", "radius": 1}).validate)
        self.assertFalse(validator({"kind": "square", "radius": 1}).validate)

    def test_composes_with_union_and_array_of(self):
        validator = array_of(union(literal("on"), literal("off")))
        self.assertTrue(validator(["on", "off"]).validate)
        self.assertFalse(validator(["on", "auto"]).validate)
        self.assertTrue(union(literal(0), string())("x").validate)


if __name__ == "__main__":
    unittest.main()
