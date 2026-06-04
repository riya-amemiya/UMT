import unittest

from src.validate import BigIntRule, bigint


class TestBigint(unittest.TestCase):
    def test_accepts_big_integer_values(self):
        validator = bigint()
        self.assertTrue(validator(0).validate)
        self.assertTrue(validator(123).validate)
        self.assertTrue(validator(-9007199254740993).validate)

    def test_rejects_non_integer_values(self):
        validator = bigint()
        self.assertFalse(validator(0.5).validate)
        self.assertFalse(validator("0").validate)
        self.assertFalse(validator(None).validate)
        self.assertFalse(validator(True).validate)

    def test_returns_configured_message_on_failure(self):
        validator = bigint([], "must be a bigint")
        self.assertEqual(validator("1").message, "must be a bigint")

    def test_applies_validation_rules(self):
        positive = BigIntRule(
            validate=lambda value: value > 0,
            message="must be positive",
        )
        validator = bigint([positive])
        self.assertTrue(validator(1).validate)
        self.assertFalse(validator(0).validate)
        self.assertEqual(validator(-1).message, "must be positive")

    def test_falls_back_to_empty_message_when_rule_has_no_message(self):
        positive = BigIntRule(validate=lambda value: value > 0)
        validator = bigint([positive])
        self.assertEqual(validator(-1).message, "")

    def test_type_tag(self):
        validator = bigint()
        self.assertEqual(validator(1).type, "bigint")
        self.assertEqual(validator("1").type, "bigint")

    def test_docstring_example(self):
        validator = bigint()
        self.assertTrue(validator(123).validate)
        self.assertFalse(validator("123").validate)
        positive = BigIntRule(
            validate=lambda value: value > 0,
            message="must be positive",
        )
        self.assertEqual(bigint([positive])(-1).message, "must be positive")


if __name__ == "__main__":
    unittest.main()
