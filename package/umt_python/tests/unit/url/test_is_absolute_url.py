import unittest

from src.url import is_absolute_url


class TestIsAbsoluteUrl(unittest.TestCase):
    def test_http_urls(self):
        self.assertTrue(is_absolute_url("http://example.com"))
        self.assertTrue(is_absolute_url("https://example.com"))

    def test_other_schemes(self):
        self.assertTrue(is_absolute_url("ftp://files.example"))
        self.assertTrue(is_absolute_url("mailto:user@host"))
        self.assertTrue(is_absolute_url("tel:+1234567890"))
        self.assertTrue(is_absolute_url("ssh://host"))

    def test_custom_schemes(self):
        self.assertTrue(is_absolute_url("custom+scheme://path"))
        self.assertTrue(is_absolute_url("my-app://deep-link"))
        self.assertTrue(is_absolute_url("x.y://test"))

    def test_relative_paths(self):
        self.assertFalse(is_absolute_url("/path/to/page"))
        self.assertFalse(is_absolute_url("relative/path"))
        self.assertFalse(is_absolute_url("./relative"))
        self.assertFalse(is_absolute_url("../parent"))

    def test_protocol_relative_urls(self):
        self.assertFalse(is_absolute_url("//example.com"))

    def test_empty_string(self):
        self.assertFalse(is_absolute_url(""))

    def test_invalid_scheme_starts(self):
        self.assertFalse(is_absolute_url("123://invalid"))
        self.assertFalse(is_absolute_url("+bad://invalid"))

    def test_uppercase_schemes(self):
        self.assertTrue(is_absolute_url("HTTP://example.com"))
        self.assertTrue(is_absolute_url("HTTPS://example.com"))
        self.assertTrue(is_absolute_url("Mailto:user@host"))

    def test_scheme_with_digits(self):
        self.assertTrue(is_absolute_url("http2://example.com"))
        self.assertTrue(is_absolute_url("a1:"))

    def test_missing_colon(self):
        self.assertFalse(is_absolute_url("http"))
        self.assertFalse(is_absolute_url(":"))

    def test_leading_whitespace(self):
        self.assertFalse(is_absolute_url(" http://example.com"))


if __name__ == "__main__":
    unittest.main()
