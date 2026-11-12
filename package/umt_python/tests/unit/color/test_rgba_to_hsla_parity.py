import unittest

from src.color import rgba_to_hsla


class TestRgbaToHslaParity(unittest.TestCase):
    def test_gray_has_no_hue_or_saturation(self):
        self.assertEqual(
            rgba_to_hsla(100, 100, 100), {"h": 0, "s": 0, "l": 39.22, "a": 1}
        )

    def test_red_is_max_with_green_at_or_above_blue(self):
        self.assertEqual(
            rgba_to_hsla(214, 54, 63), {"h": 356.63, "s": 66.12, "l": 52.55, "a": 1}
        )

    def test_red_is_max_with_green_below_blue(self):
        self.assertEqual(
            rgba_to_hsla(255, 0, 100), {"h": 336.47, "s": 100, "l": 50, "a": 1}
        )

    def test_green_is_max(self):
        self.assertEqual(
            rgba_to_hsla(161, 207, 143), {"h": 103.13, "s": 40, "l": 68.63, "a": 1}
        )
        self.assertEqual(
            rgba_to_hsla(47, 143, 124, 0.4),
            {"h": 168.13, "s": 50.53, "l": 37.25, "a": 0.4},
        )

    def test_blue_is_max(self):
        self.assertEqual(
            rgba_to_hsla(61, 6, 166), {"h": 260.63, "s": 93.02, "l": 33.73, "a": 1}
        )
        self.assertEqual(
            rgba_to_hsla(173, 216, 230), {"h": 194.74, "s": 53.27, "l": 79.02, "a": 1}
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
                rgba_to_hsla(*arguments)


if __name__ == "__main__":
    unittest.main()
