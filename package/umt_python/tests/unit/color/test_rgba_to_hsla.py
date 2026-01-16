import unittest

from src.color import rgba_to_hsla


class TestRgbaToHsla(unittest.TestCase):
    def test_basic_cases(self):
        result = rgba_to_hsla(100, 100, 100, 1)
        self.assertEqual(result["h"], 0)
        self.assertEqual(result["s"], 0)
        self.assertAlmostEqual(result["l"], 39.22, places=2)
        self.assertEqual(result["a"], 1)

    def test_primary_colors(self):
        red = rgba_to_hsla(255, 0, 0)
        self.assertEqual(red["h"], 0)
        self.assertEqual(red["s"], 100)
        self.assertEqual(red["l"], 50)

        green = rgba_to_hsla(0, 255, 0)
        self.assertEqual(green["h"], 120)
        self.assertEqual(green["s"], 100)
        self.assertEqual(green["l"], 50)

        blue = rgba_to_hsla(0, 0, 255)
        self.assertEqual(blue["h"], 240)
        self.assertEqual(blue["s"], 100)
        self.assertEqual(blue["l"], 50)

    def test_edge_cases(self):
        black = rgba_to_hsla(0, 0, 0)
        self.assertEqual(black["h"], 0)
        self.assertEqual(black["s"], 0)
        self.assertEqual(black["l"], 0)

        white = rgba_to_hsla(255, 255, 255)
        self.assertEqual(white["h"], 0)
        self.assertEqual(white["s"], 0)
        self.assertEqual(white["l"], 100)

    def test_invalid_input(self):
        with self.assertRaises(ValueError):
            rgba_to_hsla(-1, 0, 0)
        with self.assertRaises(ValueError):
            rgba_to_hsla(256, 0, 0)
        with self.assertRaises(ValueError):
            rgba_to_hsla(0, 0, 0, -0.1)
        with self.assertRaises(ValueError):
            rgba_to_hsla(0, 0, 0, 1.1)

    def test_docstring_example(self):
        result = rgba_to_hsla(100, 100, 100, 1)
        self.assertEqual(result["h"], 0)
        self.assertEqual(result["s"], 0)
        self.assertAlmostEqual(result["l"], 39.22, places=2)


if __name__ == "__main__":
    unittest.main()
