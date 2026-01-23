import unittest

from src.validate import parse_email
from src.validate.parse_email import ParseEmailOptions


class TestParseEmail(unittest.TestCase):
    def test_basic_valid_email(self):
        result = parse_email("test@example.com", ParseEmailOptions(level="basic"))
        self.assertTrue(result.valid)
        assert result.parts is not None
        self.assertEqual(result.parts.local, "test")
        self.assertEqual(result.parts.domain, "example.com")

    def test_basic_invalid_email(self):
        result = parse_email("invalid-email", ParseEmailOptions(level="basic"))
        self.assertFalse(result.valid)
        self.assertIsNone(result.parts)

    def test_rfc822_valid_email(self):
        result = parse_email("test@example.com", ParseEmailOptions(level="rfc822"))
        self.assertTrue(result.valid)
        assert result.parts is not None
        self.assertEqual(result.parts.local, "test")
        self.assertEqual(result.parts.domain, "example.com")

    def test_rfc822_quoted_local(self):
        result = parse_email(
            '"test.user"@example.com', ParseEmailOptions(level="rfc822")
        )
        self.assertTrue(result.valid)

    def test_rfc2822_valid_email(self):
        result = parse_email("test@example.com", ParseEmailOptions(level="rfc2822"))
        self.assertTrue(result.valid)
        assert result.parts is not None
        self.assertEqual(result.parts.local, "test")
        self.assertEqual(result.parts.domain, "example.com")

    def test_rfc2822_invalid_email(self):
        result = parse_email("test@example", ParseEmailOptions(level="rfc2822"))
        self.assertFalse(result.valid)

    def test_rfc5321_valid_email(self):
        result = parse_email("test@example.com", ParseEmailOptions(level="rfc5321"))
        self.assertTrue(result.valid)
        assert result.parts is not None
        self.assertEqual(result.parts.local, "test")
        self.assertEqual(result.parts.domain, "example.com")

    def test_rfc5321_ip_domain(self):
        result = parse_email("test@[192.168.1.1]", ParseEmailOptions(level="rfc5321"))
        self.assertTrue(result.valid)

    def test_rfc5322_valid_email(self):
        result = parse_email("test@example.com", ParseEmailOptions(level="rfc5322"))
        self.assertTrue(result.valid)
        assert result.parts is not None
        self.assertEqual(result.parts.local, "test")
        self.assertEqual(result.parts.domain, "example.com")

    def test_complex_local_part(self):
        result = parse_email(
            "test.user+tag@example.com", ParseEmailOptions(level="basic")
        )
        self.assertTrue(result.valid)
        assert result.parts is not None
        self.assertEqual(result.parts.local, "test.user+tag")

    def test_subdomain(self):
        result = parse_email("test@mail.example.com", ParseEmailOptions(level="basic"))
        self.assertTrue(result.valid)
        assert result.parts is not None
        self.assertEqual(result.parts.domain, "mail.example.com")

    def test_empty_email(self):
        result = parse_email("", ParseEmailOptions(level="basic"))
        self.assertFalse(result.valid)

    def test_missing_at_symbol(self):
        result = parse_email("testexample.com", ParseEmailOptions(level="basic"))
        self.assertFalse(result.valid)

    def test_missing_domain(self):
        result = parse_email("test@", ParseEmailOptions(level="basic"))
        self.assertFalse(result.valid)

    def test_docstring_example(self):
        result = parse_email("test@example.com", ParseEmailOptions(level="basic"))
        self.assertTrue(result.valid)
        assert result.parts is not None
        self.assertEqual(result.parts.local, "test")
        self.assertEqual(result.parts.domain, "example.com")


if __name__ == "__main__":
    unittest.main()
