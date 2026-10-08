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

    def test_ipod_and_mobile_variants(self):
        ipod = "Mozilla/5.0 (iPod touch; CPU iPhone OS 14_4 like Mac OS X)"
        blackberry = "BlackBerry9000/4.6.0.167 Profile/MIDP-2.0 Configuration/CLDC-1.1"
        webos = "Mozilla/5.0 (webOS/1.4.5; U; en-US) AppleWebKit/532.2"
        opera_mini = "Opera/9.80 (J2ME/MIDP; Opera Mini/9.80)"
        self.assertEqual(extract_device_from_user_agent(ipod), "mobile")
        self.assertEqual(extract_device_from_user_agent(blackberry), "mobile")
        self.assertEqual(extract_device_from_user_agent(webos), "mobile")
        self.assertEqual(extract_device_from_user_agent(opera_mini), "mobile")

    def test_macintosh_and_linux_desktop(self):
        mac = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15"
        linux = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36"
        self.assertEqual(extract_device_from_user_agent(mac), "desktop")
        self.assertEqual(extract_device_from_user_agent(linux), "desktop")

    def test_empty_and_spider(self):
        self.assertEqual(extract_device_from_user_agent(""), "other")
        spider = "Mozilla/5.0 (compatible; YandexBot/3.0; +http://yandex.com/bots)"
        self.assertEqual(extract_device_from_user_agent(spider), "bot")

    def test_case_insensitive(self):
        self.assertEqual(
            extract_device_from_user_agent(
                "MOZILLA/5.0 (IPHONE; CPU IPHONE OS 14_4 LIKE MAC OS X)"
            ),
            "mobile",
        )
        self.assertEqual(
            extract_device_from_user_agent(
                "MOZILLA/5.0 (LINUX; ANDROID 10; SM-G973F) MOBILE SAFARI"
            ),
            "mobile",
        )
        self.assertEqual(
            extract_device_from_user_agent(
                "MOZILLA/5.0 (IPAD; CPU OS 14_6 LIKE MAC OS X)"
            ),
            "tablet",
        )

    def test_docstring_example(self):
        ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64)"
        self.assertEqual(extract_device_from_user_agent(ua), "desktop")


if __name__ == "__main__":
    unittest.main()
