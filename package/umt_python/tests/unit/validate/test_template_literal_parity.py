import unittest

from src.validate import bigint, number, string, template_literal


def _custom_like(_value: object = None) -> object:
    class _Result:
        validate = True
        message = ""
        type = "custom"

    return _Result()


class TestTemplateLiteralParity(unittest.TestCase):
    def test_number_fragment_only_accepts_ascii_digits(self):
        validator = template_literal(["item-", number()])
        self.assertTrue(validator("item-12").validate)
        self.assertTrue(validator("item--1.5").validate)
        self.assertFalse(validator("item-" + chr(0x0661)).validate)
        self.assertFalse(validator("item-" + chr(0xFF11)).validate)

    def test_bigint_fragment_only_accepts_ascii_digits(self):
        validator = template_literal(["id-", bigint()])
        self.assertTrue(validator("id-12").validate)
        self.assertFalse(validator("id-" + chr(0x0661)).validate)
        self.assertFalse(validator("id-1.5").validate)

    def test_string_fragment_does_not_cross_line_terminators(self):
        validator = template_literal(["a", string(), "b"])
        self.assertTrue(validator("ab").validate)
        self.assertTrue(validator("a b").validate)
        for terminator in ("\n", "\r", chr(0x2028), chr(0x2029)):
            self.assertFalse(validator(f"a{terminator}b").validate)

    def test_unknown_fragment_does_not_cross_line_terminators(self):
        validator = template_literal(["a", _custom_like, "b"])
        self.assertTrue(validator("axb").validate)
        self.assertFalse(validator("ab").validate)
        for terminator in ("\n", "\r", chr(0x2028), chr(0x2029)):
            self.assertFalse(validator(f"a{terminator}b").validate)


if __name__ == "__main__":
    unittest.main()
