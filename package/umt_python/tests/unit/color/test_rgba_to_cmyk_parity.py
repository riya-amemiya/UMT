import unittest

from src.color import rgba_to_cmyk


class TestRgbaToCmykParity(unittest.TestCase):
    def test_matches_typescript_two_decimal_rounding(self):
        self.assertEqual(
            rgba_to_cmyk(79, 160, 104, 0.5),
            {"c": 50.63, "m": 0, "y": 35, "k": 37.25, "a": 0.5},
        )
        self.assertEqual(
            rgba_to_cmyk(193, 224, 21, 0.5),
            {"c": 13.84, "m": 0, "y": 90.63, "k": 12.16, "a": 0.5},
        )
        self.assertEqual(
            rgba_to_cmyk(12, 200, 255, 0.5),
            {"c": 95.29, "m": 21.57, "y": 0, "k": 0, "a": 0.5},
        )

    def test_black_maps_division_by_zero_to_zero(self):
        self.assertEqual(
            rgba_to_cmyk(0, 0, 0, 0.5),
            {"c": 0, "m": 0, "y": 0, "k": 100, "a": 0.5},
        )

    def test_white_and_primary(self):
        self.assertEqual(
            rgba_to_cmyk(255, 255, 255), {"c": 0, "m": 0, "y": 0, "k": 0, "a": 1}
        )
        self.assertEqual(
            rgba_to_cmyk(255, 0, 0), {"c": 0, "m": 100, "y": 100, "k": 0, "a": 1}
        )

    def test_out_of_range_values_raise(self):
        for arguments in (
            (-1, 0, 0, 1),
            (256, 0, 0, 1),
            (0, -1, 0, 1),
            (0, 256, 0, 1),
            (0, 0, -1, 1),
            (0, 0, 256, 1),
            (0, 0, 0, -0.1),
            (0, 0, 0, 1.1),
        ):
            with self.assertRaises(ValueError):
                rgba_to_cmyk(*arguments)


if __name__ == "__main__":
    unittest.main()
