import unittest

from src.ua import extract_device_from_user_agent


class TestExtractDeviceFromUserAgent(unittest.TestCase):
    def test_mobile(self):
        ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 14_6 like Mac OS X) AppleWebKit/605.1.15"
        self.assertEqual(extract_device_from_user_agent(ua), "mobile")

    def test_tablet(self):
        ua = "Mozilla/5.0 (iPad; CPU OS 14_6 like Mac OS X) AppleWebKit/605.1.15"
        self.assertEqual(extract_device_from_user_agent(ua), "tablet")

    def test_desktop(self):
        ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
        self.assertEqual(extract_device_from_user_agent(ua), "desktop")

    def test_bot(self):
        ua = "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"
        self.assertEqual(extract_device_from_user_agent(ua), "bot")

        ua = "Mozilla/5.0 (compatible; bingbot/2.0; +http://www.bing.com/bingbot.htm)"
        self.assertEqual(extract_device_from_user_agent(ua), "bot")

        ua = "Crawler/1.0"
        self.assertEqual(extract_device_from_user_agent(ua), "bot")

    def test_android_tablet(self):
        ua = "Mozilla/5.0 (Linux; Android 10; SM-T510) AppleWebKit/537.36"
        self.assertEqual(extract_device_from_user_agent(ua), "tablet")

    def test_android_mobile(self):
        ua = "Mozilla/5.0 (Linux; Android 10; SM-G973F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0 Mobile Safari/537.36"
        self.assertEqual(extract_device_from_user_agent(ua), "mobile")

    def test_other(self):
        ua = "Unknown Device"
        self.assertEqual(extract_device_from_user_agent(ua), "other")

        ua = "SomeRandomApp/1.0"
        self.assertEqual(extract_device_from_user_agent(ua), "other")

    def test_docstring_example(self):
        ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64)"
        self.assertEqual(extract_device_from_user_agent(ua), "desktop")


if __name__ == "__main__":
    unittest.main()
