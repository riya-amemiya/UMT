import unittest

from src.color import rgba_to_hexa


class TestRgbaToHexaParity(unittest.TestCase):
    def test_alpha_rounds_half_up_like_typescript(self):
        self.assertEqual(rgba_to_hexa(255, 0, 17, 0.3), "#ff00114d")
        self.assertEqual(rgba_to_hexa(255, 0, 17, 0.7), "#ff0011b3")
        self.assertEqual(rgba_to_hexa(255, 0, 17, 0.5), "#ff001180")
        self.assertEqual(rgba_to_hexa(255, 0, 17, 0.1), "#ff00111a")
        self.assertEqual(rgba_to_hexa(255, 0, 17, 0.9), "#ff0011e6")

    def test_alpha_bounds_and_default(self):
        self.assertEqual(rgba_to_hexa(255, 0, 17, 0), "#ff001100")
        self.assertEqual(rgba_to_hexa(255, 0, 17), "#ff0011ff")

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
                rgba_to_hexa(*arguments)


if __name__ == "__main__":
    unittest.main()
