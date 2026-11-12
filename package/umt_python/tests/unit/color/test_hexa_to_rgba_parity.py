import unittest

from src.color import hexa_to_rgba


class TestHexaToRgbaParity(unittest.TestCase):
    def test_three_six_and_eight_digit_codes(self):
        self.assertEqual(hexa_to_rgba("#abc"), {"r": 170, "g": 187, "b": 204, "a": 1})
        self.assertEqual(
            hexa_to_rgba("#A1b2C3"), {"r": 161, "g": 178, "b": 195, "a": 1}
        )
        self.assertEqual(
            hexa_to_rgba("#10203040"), {"r": 16, "g": 32, "b": 48, "a": 0.25}
        )
        self.assertEqual(
            hexa_to_rgba("#aabbccdd"), {"r": 170, "g": 187, "b": 204, "a": 0.87}
        )

    def test_trailing_newline_is_rejected(self):
        for value in ("#fff\n", "#ffffff\n", "#ffffffff\n"):
            with self.assertRaises(ValueError):
                hexa_to_rgba(value)

    def test_invalid_codes_are_rejected(self):
        arabic_indic_three = "\N{ARABIC-INDIC DIGIT THREE}"
        for value in ("", "fff", "#ff", "#ffff", "#ggg", "#" + arabic_indic_three * 3):
            with self.assertRaises(ValueError):
                hexa_to_rgba(value)


if __name__ == "__main__":
    unittest.main()
