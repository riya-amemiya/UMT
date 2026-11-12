import unittest

from src.color import get_contrast_ratio, hexa_to_rgba


class TestGetContrastRatio(unittest.TestCase):
    black = {"r": 0, "g": 0, "b": 0}
    white = {"r": 255, "g": 255, "b": 255, "a": 1}

    def test_black_and_white(self):
        self.assertAlmostEqual(
            get_contrast_ratio(self.black, self.white), 21, places=12
        )

    def test_argument_order(self):
        gray = {"r": 119, "g": 119, "b": 119}
        self.assertAlmostEqual(
            get_contrast_ratio(self.white, gray), 4.478089453577214, places=12
        )
        self.assertEqual(
            get_contrast_ratio(gray, self.white), get_contrast_ratio(self.white, gray)
        )

    def test_identical_colors(self):
        self.assertEqual(get_contrast_ratio(self.white, self.white), 1)
        self.assertEqual(get_contrast_ratio(self.black, self.black), 1)

    def test_saturated_colors(self):
        self.assertAlmostEqual(
            get_contrast_ratio({"r": 255, "g": 0, "b": 0}, {"r": 0, "g": 0, "b": 255}),
            2.148936170212766,
            places=12,
        )

    def test_accepts_hexa_to_rgba_output(self):
        self.assertAlmostEqual(
            get_contrast_ratio(hexa_to_rgba("#000"), hexa_to_rgba("#ffffff")),
            21,
            places=12,
        )

    def test_invalid_values(self):
        with self.assertRaises(ValueError):
            get_contrast_ratio({"r": 256, "g": 0, "b": 0}, self.white)
        with self.assertRaises(ValueError):
            get_contrast_ratio(self.black, {"r": 0, "g": 0, "b": 0, "a": 2})


if __name__ == "__main__":
    unittest.main()
