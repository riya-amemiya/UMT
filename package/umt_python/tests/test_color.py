import unittest

from src.color import (
    cmyk_to_rgba,
    hexa_to_rgba,
    hsla_to_rgba,
    rgba_to_cmyk,
    rgba_to_hexa,
    rgba_to_hsla,
)


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


class TestRgbaToHexa(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(rgba_to_hexa(0, 0, 0, 1), "#000000ff")
        self.assertEqual(rgba_to_hexa(255, 255, 255, 1), "#ffffffff")

    def test_with_alpha(self):
        self.assertEqual(rgba_to_hexa(255, 0, 0, 0.5), "#ff000080")

    def test_invalid_input(self):
        with self.assertRaises(ValueError):
            rgba_to_hexa(-1, 0, 0)
        with self.assertRaises(ValueError):
            rgba_to_hexa(256, 0, 0)
        with self.assertRaises(ValueError):
            rgba_to_hexa(0, 0, 0, -0.1)

    def test_docstring_example(self):
        self.assertEqual(rgba_to_hexa(0, 0, 0, 1), "#000000ff")


class TestHexaToRgba(unittest.TestCase):
    def test_basic_cases(self):
        result = hexa_to_rgba("#00000000")
        self.assertEqual(result["r"], 0)
        self.assertEqual(result["g"], 0)
        self.assertEqual(result["b"], 0)
        self.assertEqual(result["a"], 0.0)

    def test_shorthand(self):
        result = hexa_to_rgba("#fff")
        self.assertEqual(result["r"], 255)
        self.assertEqual(result["g"], 255)
        self.assertEqual(result["b"], 255)
        self.assertEqual(result["a"], 1.0)

    def test_six_digit(self):
        result = hexa_to_rgba("#ff0000")
        self.assertEqual(result["r"], 255)
        self.assertEqual(result["g"], 0)
        self.assertEqual(result["b"], 0)
        self.assertEqual(result["a"], 1.0)

    def test_invalid_input(self):
        with self.assertRaises(ValueError):
            hexa_to_rgba("invalid")
        with self.assertRaises(ValueError):
            hexa_to_rgba("#gg0000")

    def test_docstring_example(self):
        result = hexa_to_rgba("#00000000")
        self.assertEqual(result, {"r": 0, "g": 0, "b": 0, "a": 0.0})


class TestRgbaToCmyk(unittest.TestCase):
    def test_basic_cases(self):
        result = rgba_to_cmyk(0, 0, 0, 1)
        self.assertEqual(result["c"], 0)
        self.assertEqual(result["m"], 0)
        self.assertEqual(result["y"], 0)
        self.assertEqual(result["k"], 100)

    def test_white(self):
        result = rgba_to_cmyk(255, 255, 255, 1)
        self.assertEqual(result["c"], 0)
        self.assertEqual(result["m"], 0)
        self.assertEqual(result["y"], 0)
        self.assertEqual(result["k"], 0)

    def test_primary_colors(self):
        red = rgba_to_cmyk(255, 0, 0)
        self.assertEqual(red["c"], 0)
        self.assertEqual(red["m"], 100)
        self.assertEqual(red["y"], 100)
        self.assertEqual(red["k"], 0)

    def test_invalid_input(self):
        with self.assertRaises(ValueError):
            rgba_to_cmyk(-1, 0, 0)
        with self.assertRaises(ValueError):
            rgba_to_cmyk(256, 0, 0)

    def test_docstring_example(self):
        result = rgba_to_cmyk(0, 0, 0, 1)
        self.assertEqual(result["c"], 0)
        self.assertEqual(result["m"], 0)
        self.assertEqual(result["y"], 0)
        self.assertEqual(result["k"], 100)


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
