import unittest

from src.time import format_duration


class TestFormatDuration(unittest.TestCase):
    def test_returns_0s_for_zero(self):
        self.assertEqual(format_duration(0), "0s")
        self.assertEqual(format_duration(-0.0), "0s")

    def test_formats_single_units(self):
        self.assertEqual(format_duration(45000), "45s")
        self.assertEqual(format_duration(60000), "1m")
        self.assertEqual(format_duration(3600000), "1h")
        self.assertEqual(format_duration(86400000), "1d")
        self.assertEqual(format_duration(250), "250ms")

    def test_omits_zero_units(self):
        self.assertEqual(format_duration(5400000), "1h 30m")
        self.assertEqual(format_duration(183600000), "2d 3h")
        self.assertEqual(format_duration(86400005), "1d 5ms")

    def test_formats_every_unit(self):
        self.assertEqual(format_duration(93784005), "1d 2h 3m 4s 5ms")

    def test_does_not_cap_days(self):
        self.assertEqual(format_duration(400 * 86400000), "400d")

    def test_truncates_fractional_milliseconds(self):
        self.assertEqual(format_duration(1500.9), "1s 500ms")
        self.assertEqual(format_duration(0.5), "0s")

    def test_keeps_leading_minus_for_negative(self):
        self.assertEqual(format_duration(-90000), "-1m 30s")
        self.assertEqual(format_duration(-5400000), "-1h 30m")

    def test_returns_0s_without_sign_when_negative_truncates_to_zero(self):
        self.assertEqual(format_duration(-0.5), "0s")


if __name__ == "__main__":
    unittest.main()
