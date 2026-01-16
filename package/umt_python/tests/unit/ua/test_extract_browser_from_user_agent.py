import unittest

from src.ua import extract_browser_from_user_agent


class TestExtractBrowserFromUserAgent(unittest.TestCase):
    def test_chrome(self):
        ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"
        self.assertEqual(extract_browser_from_user_agent(ua), "chrome")

    def test_firefox(self):
        ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:89.0) Gecko/20100101 Firefox/89.0"
        self.assertEqual(extract_browser_from_user_agent(ua), "firefox")

    def test_safari(self):
        ua = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.1 Safari/605.1.15"
        self.assertEqual(extract_browser_from_user_agent(ua), "safari")

    def test_edge(self):
        ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36 Edg/91.0.864.59"
        self.assertEqual(extract_browser_from_user_agent(ua), "edge")

    def test_ie(self):
        ua = "Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; rv:11.0) like Gecko"
        self.assertEqual(extract_browser_from_user_agent(ua), "ie")

    def test_other(self):
        ua = "Unknown Browser"
        self.assertEqual(extract_browser_from_user_agent(ua), "other")

    def test_docstring_example(self):
        ua = "Mozilla/5.0 Chrome/91.0"
        self.assertEqual(extract_browser_from_user_agent(ua), "chrome")


if __name__ == "__main__":
    unittest.main()
