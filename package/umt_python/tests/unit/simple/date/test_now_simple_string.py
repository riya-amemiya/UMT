import unittest

from src.simple.date import now_simple


def _offset_hours(value):
    offset = now_simple(value).utcoffset()
    assert offset is not None
    return offset.total_seconds() / 3600


class TestNowSimpleString(unittest.TestCase):
    def test_integer_string(self):
        self.assertEqual(_offset_hours("3"), 3)

    def test_zero_padded_string(self):
        self.assertEqual(_offset_hours("09"), 9)

    def test_negative_string(self):
        self.assertEqual(_offset_hours("-5"), -5)

    def test_fractional_string(self):
        self.assertEqual(_offset_hours("5.5"), 5.5)

    def test_string_with_surrounding_whitespace(self):
        self.assertEqual(_offset_hours(" 2 "), 2)

    def test_empty_string_is_zero(self):
        self.assertEqual(_offset_hours(""), 0)

    def test_exponent_string(self):
        self.assertEqual(_offset_hours("1e1"), 10)

    def test_non_numeric_string_falls_back_to_default(self):
        self.assertEqual(_offset_hours("abc"), 9)

    def test_infinity_string_falls_back_to_default(self):
        self.assertEqual(_offset_hours("Infinity"), 9)

    def test_python_only_spelling_falls_back_to_default(self):
        self.assertEqual(_offset_hours("1_0"), 9)

    def test_integer_input_is_unchanged(self):
        self.assertEqual(_offset_hours(0), 0)


if __name__ == "__main__":
    unittest.main()
