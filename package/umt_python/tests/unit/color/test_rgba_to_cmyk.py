import unittest

from src.color import rgba_to_cmyk


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


if __name__ == "__main__":
    unittest.main()
