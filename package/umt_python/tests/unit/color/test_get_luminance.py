import unittest

from src.color import get_luminance


class TestGetLuminance(unittest.TestCase):
    def test_relative_luminance(self):
        for rgb, expected in [
            ((255, 255, 255), 1),
            ((0, 0, 0), 0),
            ((255, 0, 0), 0.2126),
            ((0, 255, 0), 0.7152),
            ((0, 0, 255), 0.0722),
            ((128, 128, 128), 0.21586050011389923),
            ((119, 119, 119), 0.184474994500441),
        ]:
            with self.subTest(rgb=rgb):
                self.assertAlmostEqual(get_luminance(*rgb), expected, places=12)

    def test_linear_segment(self):
        self.assertAlmostEqual(get_luminance(10, 10, 10), 10 / 255 / 12.92, places=15)
        self.assertAlmostEqual(
            get_luminance(1, 2, 3),
            (0.2126 * 1 + 0.7152 * 2 + 0.0722 * 3) / 255 / 12.92,
            places=15,
        )

    def test_ignores_alpha(self):
        self.assertEqual(get_luminance(128, 64, 32, 0), get_luminance(128, 64, 32))

    def test_invalid_values(self):
        for args in [
            (-1, 0, 0),
            (256, 0, 0),
            (0, -1, 0),
            (0, 256, 0),
            (0, 0, -1),
            (0, 0, 256),
            (0, 0, 0, -0.1),
            (0, 0, 0, 1.1),
        ]:
            with self.subTest(args=args), self.assertRaises(ValueError):
                get_luminance(*args)


if __name__ == "__main__":
    unittest.main()
