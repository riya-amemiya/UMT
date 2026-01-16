import unittest

from src.color import hsla_to_rgba


class TestHslaToRgba(unittest.TestCase):
    def test_basic_cases(self):
        result = hsla_to_rgba(120, 50, 50, 1)
        self.assertAlmostEqual(result["r"], 63.75, places=1)
        self.assertAlmostEqual(result["g"], 191.25, places=1)
        self.assertAlmostEqual(result["b"], 63.75, places=1)

    def test_primary_colors(self):
        red = hsla_to_rgba(0, 100, 50)
        self.assertAlmostEqual(red["r"], 255, places=0)
        self.assertAlmostEqual(red["g"], 0, places=0)
        self.assertAlmostEqual(red["b"], 0, places=0)

    def test_grayscale(self):
        gray = hsla_to_rgba(0, 0, 50)
        self.assertAlmostEqual(gray["r"], 127.5, places=1)
        self.assertAlmostEqual(gray["g"], 127.5, places=1)
        self.assertAlmostEqual(gray["b"], 127.5, places=1)

    def test_invalid_input(self):
        with self.assertRaises(ValueError):
            hsla_to_rgba(-1, 50, 50)
        with self.assertRaises(ValueError):
            hsla_to_rgba(361, 50, 50)
        with self.assertRaises(ValueError):
            hsla_to_rgba(0, -1, 50)
        with self.assertRaises(ValueError):
            hsla_to_rgba(0, 101, 50)


if __name__ == "__main__":
    unittest.main()
