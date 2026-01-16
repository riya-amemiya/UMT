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


if __name__ == "__main__":
    unittest.main()
