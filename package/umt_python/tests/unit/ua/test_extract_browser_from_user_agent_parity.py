import unittest

from src.ua import extract_browser_from_user_agent

LONG_S = "\N{LATIN SMALL LETTER LONG S}"
KELVIN = "\N{KELVIN SIGN}"
DOTLESS_I = "\N{LATIN SMALL LETTER DOTLESS I}"
DOTTED_I = "\N{LATIN CAPITAL LETTER I WITH DOT ABOVE}"


class TestExtractBrowserFromUserAgentParity(unittest.TestCase):
    def test_case_folding_is_ascii_only(self):
        self.assertEqual(extract_browser_from_user_agent(LONG_S + "afari"), "other")
        self.assertEqual(
            extract_browser_from_user_agent(KELVIN + "HTML Chrome"), "chrome"
        )
        self.assertEqual(
            extract_browser_from_user_agent("f" + DOTLESS_I + "refox"), "other"
        )
        self.assertEqual(
            extract_browser_from_user_agent("ms" + DOTTED_I + "e"), "other"
        )

    def test_ascii_case_is_ignored_for_every_browser(self):
        self.assertEqual(extract_browser_from_user_agent("EDG/1"), "edge")
        self.assertEqual(extract_browser_from_user_agent("TRIDENT"), "ie")
        self.assertEqual(extract_browser_from_user_agent("FXIOS"), "firefox")
        self.assertEqual(extract_browser_from_user_agent("OPR/1 CHROME"), "other")
        self.assertEqual(extract_browser_from_user_agent("CRIOS"), "chrome")
        self.assertEqual(extract_browser_from_user_agent("SAFARI"), "safari")
        self.assertEqual(extract_browser_from_user_agent(""), "other")


if __name__ == "__main__":
    unittest.main()
