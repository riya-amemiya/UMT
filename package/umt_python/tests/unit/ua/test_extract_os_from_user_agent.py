import unittest

from src.ua import extract_os_from_user_agent


class TestExtractOsFromUserAgent(unittest.TestCase):
    def test_windows(self):
        ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
        self.assertEqual(extract_os_from_user_agent(ua), "windows")

    def test_macos(self):
        ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15"
        self.assertEqual(extract_os_from_user_agent(ua), "macos")

    def test_linux(self):
        ua = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36"
        self.assertEqual(extract_os_from_user_agent(ua), "linux")

    def test_ios(self):
        ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/605.1.15"
        self.assertEqual(extract_os_from_user_agent(ua), "ios")

    def test_android(self):
        ua = "Mozilla/5.0 (Linux; Android 11; SM-G998B) AppleWebKit/537.36"
        self.assertEqual(extract_os_from_user_agent(ua), "android")

    def test_other(self):
        ua = "Unknown OS"
        self.assertEqual(extract_os_from_user_agent(ua), "other")

    def test_docstring_example(self):
        ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)"
        self.assertEqual(extract_os_from_user_agent(ua), "macos")


if __name__ == "__main__":
    unittest.main()
