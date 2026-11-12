import unittest

from src.ip import get_ip_class


class TestGetIpClassParity(unittest.TestCase):
    def test_first_octet_stops_at_first_non_digit(self):
        self.assertEqual(get_ip_class("10abc.0.0.1"), "A")
        self.assertEqual(get_ip_class("1e2.0.0.1"), "A")
        self.assertEqual(get_ip_class("12 3.0.0.1"), "A")
        self.assertEqual(get_ip_class("1_0.0.0.1"), "A")
        self.assertEqual(get_ip_class("300abc.0.0.1"), "")

    def test_leading_javascript_whitespace_is_skipped(self):
        self.assertEqual(get_ip_class(" 10.0.0.1"), "A")
        self.assertEqual(get_ip_class("\t127.0.0.1"), "A")
        self.assertEqual(get_ip_class("\N{NO-BREAK SPACE}200.1.1.1"), "C")
        self.assertEqual(get_ip_class("\N{ZERO WIDTH NO-BREAK SPACE}200.1.1.1"), "C")
        self.assertEqual(get_ip_class("\N{IDEOGRAPHIC SPACE}224.1.1.1"), "D")
        self.assertEqual(get_ip_class("\N{LINE SEPARATOR}240.1.1.1"), "E")

    def test_non_javascript_whitespace_is_not_skipped(self):
        self.assertEqual(get_ip_class("\x85240.1.1.1"), "")
        self.assertEqual(get_ip_class("\N{MONGOLIAN VOWEL SEPARATOR}240.1.1.1"), "")

    def test_sign_handling(self):
        self.assertEqual(get_ip_class("+10.0.0.1"), "A")
        self.assertEqual(get_ip_class("-5.0.0.1"), "")
        self.assertEqual(get_ip_class("-0.0.0.0"), "")
        self.assertEqual(get_ip_class("+ 5.0.0.1"), "")
        self.assertEqual(get_ip_class("+.0.0.1"), "")

    def test_non_ascii_digits_are_not_numbers(self):
        self.assertEqual(
            get_ip_class("\N{FULLWIDTH DIGIT ONE}\N{FULLWIDTH DIGIT ZERO}.0.0.1"), ""
        )
        self.assertEqual(get_ip_class("\N{ARABIC-INDIC DIGIT THREE}.0.0.1"), "")

    def test_missing_or_oversized_first_octet(self):
        self.assertEqual(get_ip_class(".0.0.1"), "")
        self.assertEqual(get_ip_class("0x10.0.0.1"), "")
        self.assertEqual(get_ip_class("99999999999999999999.0.0.1"), "")
        self.assertEqual(get_ip_class("256.0.0.1"), "")

    def test_class_boundaries(self):
        self.assertEqual(get_ip_class("010.0.0.1"), "A")
        self.assertEqual(get_ip_class("127.0.0.1"), "A")
        self.assertEqual(get_ip_class("128.0.0.1"), "B")
        self.assertEqual(get_ip_class("191.0.0.1"), "B")
        self.assertEqual(get_ip_class("192.0.0.1"), "C")
        self.assertEqual(get_ip_class("223.0.0.1"), "C")
        self.assertEqual(get_ip_class("224.0.0.1"), "D")
        self.assertEqual(get_ip_class("239.0.0.1"), "D")
        self.assertEqual(get_ip_class("240.0.0.1"), "E")
        self.assertEqual(get_ip_class("255.0.0.1"), "E")

    def test_empty_and_wrong_segment_count(self):
        self.assertEqual(get_ip_class(""), "")
        self.assertEqual(get_ip_class("10.0.0"), "")
        self.assertEqual(get_ip_class("10.0.0.1.1"), "")


if __name__ == "__main__":
    unittest.main()
