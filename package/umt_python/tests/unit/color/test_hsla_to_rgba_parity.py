import unittest

from src.color import hsla_to_rgba


class TestHslaToRgbaParity(unittest.TestCase):
    def test_matches_typescript_two_decimal_rounding(self):
        self.assertEqual(
            hsla_to_rgba(0, 1, 70, 0.1),
            {"r": 179.27, "g": 177.74, "b": 177.74, "a": 0.1},
        )
        self.assertEqual(
            hsla_to_rgba(0, 50, 75, 0.25),
            {"r": 223.13, "g": 159.38, "b": 159.38, "a": 0.25},
        )

    def test_lightness_below_half(self):
        self.assertEqual(
            hsla_to_rgba(120, 50, 50, 1),
            {"r": 63.75, "g": 191.25, "b": 63.75, "a": 1},
        )
        self.assertEqual(
            hsla_to_rgba(90, 25, 20),
            {"r": 51, "g": 63.75, "b": 38.25, "a": 1},
        )

    def test_lightness_at_or_above_half(self):
        self.assertEqual(
            hsla_to_rgba(210, 40, 70, 0.3),
            {"r": 147.9, "g": 178.5, "b": 209.1, "a": 0.3},
        )

    def test_every_hue_segment(self):
        self.assertEqual(
            hsla_to_rgba(30, 100, 50), {"r": 255, "g": 127.5, "b": 0, "a": 1}
        )
        self.assertEqual(
            hsla_to_rgba(200, 100, 50), {"r": 0, "g": 170, "b": 255, "a": 1}
        )
        self.assertEqual(
            hsla_to_rgba(270, 60, 40), {"r": 102, "g": 40.8, "b": 163.2, "a": 1}
        )

    def test_hue_360_wraps_to_zero(self):
        self.assertEqual(hsla_to_rgba(360, 100, 50), {"r": 255, "g": 0, "b": 0, "a": 1})

    def test_zero_saturation_is_gray(self):
        self.assertEqual(
            hsla_to_rgba(0, 0, 50), {"r": 127.5, "g": 127.5, "b": 127.5, "a": 1}
        )

    def test_fractional_hue_uses_typescript_digit_remainder(self):
        self.assertEqual(
            hsla_to_rgba(120.5, 50, 50),
            {"r": 63.75, "g": 191.25, "b": 74.38, "a": 1},
        )

    def test_out_of_range_values_raise(self):
        for arguments in (
            (-1, 50, 50, 1),
            (361, 50, 50, 1),
            (0, -1, 50, 1),
            (0, 101, 50, 1),
            (0, 50, -1, 1),
            (0, 50, 101, 1),
            (0, 50, 50, -0.1),
            (0, 50, 50, 1.1),
        ):
            with self.assertRaises(ValueError):
                hsla_to_rgba(*arguments)


if __name__ == "__main__":
    unittest.main()
