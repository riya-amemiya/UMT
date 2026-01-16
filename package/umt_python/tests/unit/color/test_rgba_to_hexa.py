import unittest

from src.color import rgba_to_hexa


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


if __name__ == "__main__":
    unittest.main()
