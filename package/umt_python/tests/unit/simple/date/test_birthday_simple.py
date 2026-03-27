import unittest

from src.simple.date import birthday_simple


class TestBirthdaySimple(unittest.TestCase):
    """Tests ported from Rust: test_birthday_simple.rs"""

    def test_string_hyphen(self):
        age = birthday_simple("2000-01-01")
        self.assertGreaterEqual(age, 0)

    def test_string_colon(self):
        age = birthday_simple("2000:01:01")
        self.assertGreaterEqual(age, 0)

    def test_string_slash(self):
        age = birthday_simple("2000/01/01")
        self.assertGreaterEqual(age, 0)

    def test_all_formats_return_same_age(self):
        age_hyphen = birthday_simple("2000-01-01")
        age_colon = birthday_simple("2000:01:01")
        age_slash = birthday_simple("2000/01/01")
        self.assertEqual(age_hyphen, age_colon)
        self.assertEqual(age_colon, age_slash)

    def test_dict_input(self):
        age = birthday_simple({"year": 2000, "mon": 1, "day": 1})
        self.assertGreaterEqual(age, 0)

    def test_future_birthday_returns_zero(self):
        age = birthday_simple("2099-01-01")
        self.assertEqual(age, 0)

    def test_invalid_string_returns_zero(self):
        age = birthday_simple("invalid")
        self.assertEqual(age, 0)
