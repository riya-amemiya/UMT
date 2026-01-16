import unittest

from src.ua import parse_user_agent


class TestParseUserAgent(unittest.TestCase):
    def test_basic_cases(self):
        ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"
        result = parse_user_agent(ua)
        self.assertEqual(result["browser"], "chrome")
        self.assertEqual(result["os"], "macos")
        self.assertEqual(result["device"], "desktop")

    def test_mobile_device(self):
        ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.1 Mobile/15E148 Safari/604.1"
        result = parse_user_agent(ua)
        self.assertEqual(result["os"], "ios")
        self.assertEqual(result["device"], "mobile")

    def test_android_device(self):
        ua = "Mozilla/5.0 (Linux; Android 11; SM-G998B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.120 Mobile Safari/537.36"
        result = parse_user_agent(ua)
        self.assertEqual(result["browser"], "chrome")
        self.assertEqual(result["os"], "android")
        self.assertEqual(result["device"], "mobile")

    def test_docstring_example(self):
        ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) Chrome/91.0"
        result = parse_user_agent(ua)
        self.assertEqual(result["browser"], "chrome")
        self.assertEqual(result["os"], "macos")
        self.assertEqual(result["device"], "desktop")


if __name__ == "__main__":
    unittest.main()
