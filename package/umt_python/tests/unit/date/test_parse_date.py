import unittest
from datetime import datetime, timedelta, timezone

from src.date import date_format, parse_date


class TestParseDate(unittest.TestCase):
    def test_parses_date_in_local_time(self):
        self.assertEqual(parse_date("2025-04-04", "YYYY-MM-DD"), datetime(2025, 4, 4))

    def test_parses_full_date_time_with_milliseconds(self):
        self.assertEqual(
            parse_date("2023/06/10 15:30:45.123", "YYYY/MM/DD HH:mm:ss.SSS"),
            datetime(2023, 6, 10, 15, 30, 45, 123000),
        )

    def test_parses_tokens_without_leading_zeros(self):
        self.assertEqual(
            parse_date("23-6-1 5:7:9", "YY-M-D H:m:s"),
            datetime(2023, 6, 1, 5, 7, 9),
        )

    def test_reads_yy_as_2000_to_2099(self):
        self.assertEqual(parse_date("99", "YY"), datetime(2099, 1, 1))
        self.assertEqual(parse_date("00", "YY"), datetime(2000, 1, 1))

    def test_defaults_missing_fields(self):
        self.assertEqual(parse_date("10:20", "HH:mm"), datetime(1970, 1, 1, 10, 20))
        self.assertEqual(parse_date("", ""), datetime(1970, 1, 1))

    def test_keeps_years_below_100(self):
        self.assertEqual(parse_date("0050-03-01", "YYYY-MM-DD"), datetime(50, 3, 1))

    def test_returns_none_for_year_zero(self):
        self.assertIsNone(parse_date("0000-01-01", "YYYY-MM-DD"))

    def test_converts_12_hour_clock_with_meridiem(self):
        self.assertEqual(parse_date("3:30 PM", "h:mm A"), datetime(1970, 1, 1, 15, 30))
        self.assertEqual(parse_date("03:30 am", "hh:mm a"), datetime(1970, 1, 1, 3, 30))
        self.assertEqual(parse_date("12:00 AM", "hh:mm A"), datetime(1970, 1, 1, 0, 0))
        self.assertEqual(parse_date("12:00 pm", "hh:mm a"), datetime(1970, 1, 1, 12, 0))

    def test_uses_12_hour_values_as_written_without_meridiem(self):
        self.assertEqual(parse_date("12", "hh"), datetime(1970, 1, 1, 12))
        self.assertEqual(parse_date("7", "h"), datetime(1970, 1, 1, 7))

    def test_ignores_meridiem_for_24_hour_tokens(self):
        self.assertEqual(parse_date("15 AM", "HH A"), datetime(1970, 1, 1, 15))

    def test_applies_z_offset(self):
        result = parse_date("2025-04-04T15:30:00+09:00", "YYYY-MM-DDTHH:mm:ssZ")
        self.assertEqual(
            result, datetime(2025, 4, 4, 15, 30, tzinfo=timezone(timedelta(hours=9)))
        )
        self.assertEqual(result, datetime(2025, 4, 4, 6, 30, tzinfo=timezone.utc))
        self.assertEqual(
            parse_date("2025-04-04 01:15 -05:30", "YYYY-MM-DD HH:mm Z"),
            datetime(2025, 4, 4, 6, 45, tzinfo=timezone.utc),
        )

    def test_applies_zz_offset(self):
        self.assertEqual(
            parse_date("2025-04-04 00:00 +0000", "YYYY-MM-DD HH:mm ZZ"),
            datetime(2025, 4, 4, tzinfo=timezone.utc),
        )
        self.assertEqual(
            parse_date("2025-01-01 00:00 +0100", "YYYY-MM-DD HH:mm ZZ"),
            datetime(2024, 12, 31, 23, tzinfo=timezone.utc),
        )

    def test_round_trips_with_date_format(self):
        pattern = "YYYY-MM-DD HH:mm:ss.SSS Z"
        date = datetime(2023, 6, 10, 15, 30, 45, 123000)
        result = parse_date(date_format(date, pattern), pattern)
        self.assertIsNotNone(result)
        assert result is not None
        self.assertEqual(result.timestamp(), date.timestamp())

    def test_matches_escaped_and_literal_text(self):
        self.assertEqual(
            parse_date("Year: 2023 (06)", "[Year:] YYYY (MM)"), datetime(2023, 6, 1)
        )
        self.assertEqual(parse_date("2023.06.10", "YYYY.MM.DD"), datetime(2023, 6, 10))
        self.assertIsNone(parse_date("2023x06x10", "YYYY.MM.DD"))

    def test_accepts_matching_day_of_week(self):
        self.assertEqual(
            parse_date("2025-04-21 1", "YYYY-MM-DD d"), datetime(2025, 4, 21)
        )

    def test_rejects_mismatching_day_of_week(self):
        self.assertIsNone(parse_date("2025-04-21 2", "YYYY-MM-DD d"))

    def test_last_occurrence_wins(self):
        self.assertEqual(parse_date("2024 2025", "YYYY YYYY"), datetime(2025, 1, 1))

    def test_returns_none_when_string_does_not_match(self):
        self.assertIsNone(parse_date("2025-4-04", "YYYY-MM-DD"))
        self.assertIsNone(parse_date("2025-04-04 extra", "YYYY-MM-DD"))
        self.assertIsNone(parse_date("2025-04-04\n", "YYYY-MM-DD"))
        self.assertIsNone(parse_date("abcd-04-04", "YYYY-MM-DD"))
        self.assertIsNone(parse_date("10:00 XM", "hh:mm A"))
        self.assertIsNone(parse_date("10:00 PM", "hh:mm a"))

    def test_rejects_non_ascii_digits(self):
        arabic_indic_2025 = "".join(chr(code) for code in (1634, 1632, 1634, 1637))
        self.assertIsNone(parse_date(arabic_indic_2025, "YYYY"))

    def test_returns_none_for_out_of_range_fields(self):
        cases = [
            ("2025-13-01", "YYYY-MM-DD"),
            ("2025-00-01", "YYYY-MM-DD"),
            ("2025-04-00", "YYYY-MM-DD"),
            ("2025-04-31", "YYYY-MM-DD"),
            ("2025-02-29", "YYYY-MM-DD"),
            ("24:00", "HH:mm"),
            ("10:60", "HH:mm"),
            ("10:00:60", "HH:mm:ss"),
            ("00 AM", "hh A"),
            ("13 PM", "hh A"),
            ("10:00 +24:00", "HH:mm Z"),
            ("10:00 +0960", "HH:mm ZZ"),
        ]
        for string, pattern in cases:
            with self.subTest(string=string, pattern=pattern):
                self.assertIsNone(parse_date(string, pattern))

    def test_accepts_february_29_in_leap_year(self):
        self.assertEqual(parse_date("2024-02-29", "YYYY-MM-DD"), datetime(2024, 2, 29))

    def test_opening_bracket_inside_escaped_text_is_literal(self):
        self.assertEqual(parse_date("[xy", "[x[y]"), datetime(1970, 1, 1))
        self.assertIsNone(parse_date("x[y", "[x[y]"))


if __name__ == "__main__":
    unittest.main()
