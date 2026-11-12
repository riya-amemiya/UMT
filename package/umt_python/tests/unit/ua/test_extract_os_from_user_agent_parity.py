import unittest

from src.ua import extract_os_from_user_agent

LONG_S = "\N{LATIN SMALL LETTER LONG S}"
DOTLESS_I = "\N{LATIN SMALL LETTER DOTLESS I}"
DOTTED_I = "\N{LATIN CAPITAL LETTER I WITH DOT ABOVE}"


class TestExtractOsFromUserAgentParity(unittest.TestCase):
    def test_case_folding_is_ascii_only(self):
        self.assertEqual(extract_os_from_user_agent("w" + DOTLESS_I + "ndows"), "other")
        self.assertEqual(extract_os_from_user_agent(DOTTED_I + "phone"), "other")
        self.assertEqual(extract_os_from_user_agent("mac o" + LONG_S + " x"), "other")
        self.assertEqual(extract_os_from_user_agent("l" + DOTLESS_I + "nux"), "other")
        self.assertEqual(extract_os_from_user_agent("andro" + DOTLESS_I + "d"), "other")

    def test_ascii_case_is_ignored_for_every_os(self):
        self.assertEqual(extract_os_from_user_agent("IPOD"), "ios")
        self.assertEqual(extract_os_from_user_agent("ANDROID"), "android")
        self.assertEqual(extract_os_from_user_agent("MAC OS X"), "macos")
        self.assertEqual(extract_os_from_user_agent("WIN32"), "windows")
        self.assertEqual(extract_os_from_user_agent("LINUX"), "linux")
        self.assertEqual(extract_os_from_user_agent(""), "other")


if __name__ == "__main__":
    unittest.main()
