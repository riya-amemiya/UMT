import unittest

from src.validate import number, string, template_literal


class _BooleanLikeResult:
    def __init__(self) -> None:
        self.validate = True
        self.message = ""
        self.type = "boolean"


def _boolean_like(_value: object = None) -> _BooleanLikeResult:
    return _BooleanLikeResult()


class _BigIntLikeResult:
    def __init__(self) -> None:
        self.validate = True
        self.message = ""
        self.type = "bigint"


def _bigint_like(_value: object = None) -> _BigIntLikeResult:
    return _BigIntLikeResult()


class _CustomResult:
    def __init__(self) -> None:
        self.validate = True
        self.message = ""
        self.type = "custom"


def _custom_like(_value: object = None) -> _CustomResult:
    return _CustomResult()


class TestTemplateLiteral(unittest.TestCase):
    def test_accepts_strings_matching_the_template(self):
        validator = template_literal(["hello, ", string(), "!"])
        self.assertTrue(validator("hello, world!").validate)
        self.assertTrue(validator("hello, John!").validate)

    def test_rejects_strings_that_do_not_match_the_template(self):
        validator = template_literal(["hello, ", string(), "!"])
        self.assertFalse(validator("hi, world!").validate)
        self.assertFalse(validator("hello, world").validate)

    def test_rejects_non_string_values(self):
        validator = template_literal(["hello, ", string(), "!"])
        self.assertFalse(validator(42).validate)

    def test_returns_the_configured_message_on_failure(self):
        validator = template_literal(["user-", number()], "must be user-<number>")
        self.assertEqual(validator("user-abc").message, "must be user-<number>")

    def test_supports_number_validators(self):
        validator = template_literal(["item-", number()])
        self.assertTrue(validator("item-1").validate)
        self.assertTrue(validator("item-100").validate)
        self.assertTrue(validator("item-1.5").validate)
        self.assertFalse(validator("item-abc").validate)

    def test_supports_boolean_validators(self):
        validator = template_literal(["flag-", _boolean_like])
        self.assertTrue(validator("flag-true").validate)
        self.assertTrue(validator("flag-false").validate)
        self.assertFalse(validator("flag-yes").validate)

    def test_supports_bigint_validators(self):
        validator = template_literal(["big-", _bigint_like])
        self.assertTrue(validator("big-1").validate)
        self.assertTrue(validator("big-100").validate)
        self.assertFalse(validator("big-abc").validate)

    def test_escapes_special_regex_characters_in_literal_parts(self):
        validator = template_literal(["[$]", string(), "."])
        self.assertTrue(validator("[$]anything.").validate)
        self.assertFalse(validator("xanythingy").validate)

    def test_falls_back_to_a_permissive_pattern_for_unknown_tags(self):
        validator = template_literal(["prefix-", _custom_like])
        self.assertTrue(validator("prefix-anything").validate)
        self.assertFalse(validator("noprefix").validate)

    def test_preserves_the_validated_value_through_the_type_field(self):
        validator = template_literal(["user-", number()])
        self.assertEqual(validator("user-1").type, "user-1")
        self.assertEqual(validator("user-abc").type, "user-abc")
        self.assertEqual(validator(42).type, 42)

    def test_docstring_example(self):
        validator = template_literal(["hello, ", string(), "!"])
        self.assertTrue(validator("hello, world!").validate)
        self.assertFalse(validator("hi, world!").validate)
        item = template_literal(["item-", number()])
        self.assertTrue(item("item-1.5").validate)
        self.assertFalse(item("item-abc").validate)


if __name__ == "__main__":
    unittest.main()
