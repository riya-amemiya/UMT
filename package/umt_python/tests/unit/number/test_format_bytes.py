import unittest

from src.number import format_bytes


class TestFormatBytes(unittest.TestCase):
    def test_formats_zero_as_bytes(self):
        self.assertEqual(format_bytes(0), "0 B")

    def test_keeps_values_below_1024_in_bytes(self):
        self.assertEqual(format_bytes(1), "1 B")
        self.assertEqual(format_bytes(512), "512 B")
        self.assertEqual(format_bytes(1023), "1023 B")

    def test_keeps_fractional_bytes_below_1_in_bytes(self):
        self.assertEqual(format_bytes(0.5), "0.5 B")

    def test_uses_each_unit_at_its_boundary(self):
        self.assertEqual(format_bytes(1024), "1 KB")
        self.assertEqual(format_bytes(1024**2), "1 MB")
        self.assertEqual(format_bytes(1024**3), "1 GB")
        self.assertEqual(format_bytes(1024**4), "1 TB")
        self.assertEqual(format_bytes(1024**5), "1 PB")

    def test_stays_in_pb_for_1024_pb_or_more(self):
        self.assertEqual(format_bytes(1024**6), "1024 PB")

    def test_rounds_to_two_decimals_by_default(self):
        self.assertEqual(format_bytes(1536), "1.5 KB")
        self.assertEqual(format_bytes(1234567), "1.18 MB")
        self.assertEqual(format_bytes(1048575), "1024 KB")

    def test_respects_custom_decimals(self):
        self.assertEqual(format_bytes(1234567, 0), "1 MB")
        self.assertEqual(format_bytes(1234567, 1), "1.2 MB")
        self.assertEqual(format_bytes(1234567, 4), "1.1774 MB")

    def test_formats_negative_values(self):
        self.assertEqual(format_bytes(-2048), "-2 KB")
        self.assertEqual(format_bytes(-1536), "-1.5 KB")

    def test_does_not_print_negative_zero(self):
        self.assertEqual(format_bytes(-0.0), "0 B")
        self.assertEqual(format_bytes(-0.001), "0 B")


if __name__ == "__main__":
    unittest.main()
