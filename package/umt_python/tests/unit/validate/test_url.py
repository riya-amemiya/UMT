import unittest

from src.validate import string, url


class TestUrl(unittest.TestCase):
    def test_accepts_absolute_urls(self):
        validator = string([url()])
        for value in (
            "https://example.com",
            "http://localhost:3000/path?q=1",
            "ftp://files.example",
            "mailto:user@host",
            "HTTPS://EXAMPLE.COM",
        ):
            self.assertTrue(validator(value).validate)

    def test_rejects_relative_or_malformed_urls(self):
        validator = string([url("must be a URL")])
        for value in (
            "/path/to/page",
            "relative/path",
            "",
            "1http://example.com",
            "//example.com",
        ):
            result = validator(value)
            self.assertFalse(result.validate)
            self.assertEqual(result.message, "must be a URL")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(string([url()])("example.com").message, "")


if __name__ == "__main__":
    unittest.main()
