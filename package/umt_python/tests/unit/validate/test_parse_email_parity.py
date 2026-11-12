import unittest

from src.validate import ParseEmailOptions, parse_email

LEVELS = ("basic", "rfc822", "rfc2822", "rfc5321", "rfc5322")


class TestParseEmailParity(unittest.TestCase):
    def test_rejects_inputs_longer_than_320_characters(self):
        local = "a" * 64
        domain = ".".join(["b" * 60] * 4) + ".com"
        accepted = f"{local}@{domain}"
        self.assertLessEqual(len(accepted), 320)
        self.assertTrue(parse_email(accepted, ParseEmailOptions(level="basic")).valid)

        long_domain = ".".join(["b" * 60] * 5) + ".com"
        rejected = f"{local}@{long_domain}"
        self.assertGreater(len(rejected), 320)
        for level in LEVELS:
            result = parse_email(rejected, ParseEmailOptions(level=level))
            self.assertFalse(result.valid, level)
            self.assertIsNone(result.parts)

    def test_measures_the_length_limit_in_utf16_code_units(self):
        comment = "(" + chr(0x1F600) * 155 + ")"
        accepted = f"{comment}a@b.cd"
        self.assertTrue(parse_email(accepted, ParseEmailOptions(level="rfc822")).valid)
        rejected = f"({chr(0x1F600) * 158})a@b.cd"
        self.assertLess(len(rejected), 320)
        self.assertFalse(parse_email(rejected, ParseEmailOptions(level="rfc822")).valid)

    def test_rejects_a_trailing_newline(self):
        for level in ("basic", "rfc2822", "rfc5321", "rfc5322"):
            result = parse_email("user@example.com\n", ParseEmailOptions(level=level))
            self.assertFalse(result.valid, level)

    def test_rfc822_treats_a_trailing_newline_as_whitespace(self):
        result = parse_email("user@example.com\n", ParseEmailOptions(level="rfc822"))
        self.assertTrue(result.valid)
        assert result.parts is not None
        self.assertEqual(result.parts.domain, "example.com")

    def test_rfc5322_rejects_line_terminators_anywhere(self):
        options = ParseEmailOptions(level="rfc5322")
        self.assertTrue(parse_email(" user@example.com ", options).valid)
        for terminator in ("\n", "\r", chr(0x2028), chr(0x2029)):
            self.assertFalse(
                parse_email(f"{terminator}user@example.com", options).valid
            )

    def test_uses_javascript_whitespace_for_folding(self):
        options = ParseEmailOptions(level="rfc822")
        self.assertTrue(parse_email(chr(0xFEFF) + "user@example.com", options).valid)
        self.assertFalse(parse_email(chr(0x85) + "user@example.com", options).valid)
        self.assertFalse(parse_email(chr(0x1C) + "user@example.com", options).valid)

    def test_strips_comments_from_the_parsed_parts(self):
        result = parse_email(
            "(comment)user@example.com", ParseEmailOptions(level="rfc5322")
        )
        self.assertTrue(result.valid)
        assert result.parts is not None
        self.assertEqual(result.parts.local, "user")
        self.assertEqual(result.parts.domain, "example.com")

    def test_rfc5321_limits_the_local_part_to_64_characters(self):
        options = ParseEmailOptions(level="rfc5321")
        self.assertTrue(parse_email("a" * 64 + "@example.com", options).valid)
        self.assertFalse(parse_email("a" * 65 + "@example.com", options).valid)

    def test_counts_quoted_characters_in_utf16_code_units(self):
        accepted = '"' + chr(0x1F600) * 31 + '"@example.com'
        rejected = '"' + chr(0x1F600) * 32 + '"@example.com'
        for level in ("rfc822", "rfc5321", "rfc5322"):
            options = ParseEmailOptions(level=level)
            result = parse_email(accepted, options)
            self.assertTrue(result.valid, level)
            assert result.parts is not None
            self.assertEqual(result.parts.local, '"' + chr(0x1F600) * 31 + '"')
            self.assertEqual(result.parts.domain, "example.com")
            self.assertFalse(parse_email(rejected, options).valid, level)

    def test_returns_parts_located_after_a_non_bmp_character(self):
        result = parse_email(
            f"({chr(0x1F600)})user@example ({chr(0x1F600)}) . com",
            ParseEmailOptions(level="rfc5322"),
        )
        self.assertTrue(result.valid)
        assert result.parts is not None
        self.assertEqual(result.parts.local, "user")
        self.assertEqual(result.parts.domain, f"example ({chr(0x1F600)}) . com")

    def test_keeps_bmp_only_non_ascii_input_unchanged(self):
        result = parse_email(
            f"({chr(0x3042)})user@example.com", ParseEmailOptions(level="rfc822")
        )
        self.assertTrue(result.valid)
        assert result.parts is not None
        self.assertEqual(result.parts.local, "user")
        self.assertEqual(result.parts.domain, "example.com")

    def test_rejects_consecutive_dots_at_strict_levels(self):
        for level in ("rfc2822", "rfc5321"):
            result = parse_email("a..b@example.com", ParseEmailOptions(level=level))
            self.assertFalse(result.valid, level)


if __name__ == "__main__":
    unittest.main()
