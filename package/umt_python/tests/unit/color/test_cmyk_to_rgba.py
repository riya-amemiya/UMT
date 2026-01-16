import unittest

from src.color import cmyk_to_rgba


class TestCmykToRgba(unittest.TestCase):
    def test_basic_cases(self):
        result = cmyk_to_rgba(0, 0, 0, 100)
        self.assertEqual(result["r"], 0)
        self.assertEqual(result["g"], 0)
        self.assertEqual(result["b"], 0)

    def test_white(self):
        result = cmyk_to_rgba(0, 0, 0, 0)
        self.assertEqual(result["r"], 255)
        self.assertEqual(result["g"], 255)
        self.assertEqual(result["b"], 255)

    def test_primary_colors(self):
        red = cmyk_to_rgba(0, 100, 100, 0)
        self.assertEqual(red["r"], 255)
        self.assertEqual(red["g"], 0)
        self.assertEqual(red["b"], 0)

    def test_clamping(self):
        result = cmyk_to_rgba(150, 150, 150, 150)
        self.assertEqual(result["r"], 0)
        self.assertEqual(result["g"], 0)
        self.assertEqual(result["b"], 0)


if __name__ == "__main__":
    unittest.main()
