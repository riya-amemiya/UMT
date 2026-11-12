import unittest

from src.ua import extract_device_from_user_agent

LONG_S = "\N{LATIN SMALL LETTER LONG S}"
KELVIN = "\N{KELVIN SIGN}"
DOTLESS_I = "\N{LATIN SMALL LETTER DOTLESS I}"
DOTTED_I = "\N{LATIN CAPITAL LETTER I WITH DOT ABOVE}"


class TestExtractDeviceFromUserAgentParity(unittest.TestCase):
    def test_case_folding_is_ascii_only(self):
        self.assertEqual(
            extract_device_from_user_agent("blac" + KELVIN + "berry"), "other"
        )
        self.assertEqual(
            extract_device_from_user_agent("w" + DOTLESS_I + "ndows"), "other"
        )
        self.assertEqual(extract_device_from_user_agent(DOTTED_I + "phone"), "other")
        self.assertEqual(extract_device_from_user_agent(LONG_S + "pider"), "other")
        self.assertEqual(
            extract_device_from_user_agent(
                "andro" + DOTLESS_I + "d " + DOTTED_I + "pad"
            ),
            "other",
        )

    def test_ascii_case_is_ignored_for_every_device(self):
        self.assertEqual(extract_device_from_user_agent("GOOGLEBOT"), "bot")
        self.assertEqual(extract_device_from_user_agent("OPERA MINI"), "mobile")
        self.assertEqual(extract_device_from_user_agent("ANDROID MOBILE"), "mobile")
        self.assertEqual(extract_device_from_user_agent("ANDROID"), "tablet")
        self.assertEqual(extract_device_from_user_agent("IPAD"), "tablet")
        self.assertEqual(extract_device_from_user_agent("MACINTOSH"), "desktop")
        self.assertEqual(extract_device_from_user_agent(""), "other")


if __name__ == "__main__":
    unittest.main()
