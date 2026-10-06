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
        iphone = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/605.1.15"
        ipad = "Mozilla/5.0 (iPad; CPU OS 14_4 like Mac OS X) AppleWebKit/605.1.15"
        ipod = "Mozilla/5.0 (iPod touch; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15"
        self.assertEqual(extract_os_from_user_agent(iphone), "ios")
        self.assertEqual(extract_os_from_user_agent(ipad), "ios")
        self.assertEqual(extract_os_from_user_agent(ipod), "ios")

    def test_android(self):
        ua = "Mozilla/5.0 (Linux; Android 11; SM-G998B) AppleWebKit/537.36"
        self.assertEqual(extract_os_from_user_agent(ua), "android")

    def test_win32(self):
        ua = "Mozilla/5.0 (Win32; x86) AppleWebKit/537.36"
        self.assertEqual(extract_os_from_user_agent(ua), "windows")

    def test_other(self):
        ua = "Unknown OS"
        self.assertEqual(extract_os_from_user_agent(ua), "other")

    def test_empty_and_bot(self):
        self.assertEqual(extract_os_from_user_agent(""), "other")
        bot = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"
        self.assertEqual(extract_os_from_user_agent(bot), "other")

    def test_case_insensitive(self):
        self.assertEqual(
            extract_os_from_user_agent(
                "MOZILLA/5.0 (IPHONE; CPU IPHONE OS 14_4 LIKE MAC OS X)"
            ),
            "ios",
        )
        self.assertEqual(
            extract_os_from_user_agent("MOZILLA/5.0 (LINUX; ANDROID 11; SM-G998B)"),
            "android",
        )

    def test_docstring_example(self):
        ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)"
        self.assertEqual(extract_os_from_user_agent(ua), "macos")


if __name__ == "__main__":
    unittest.main()
