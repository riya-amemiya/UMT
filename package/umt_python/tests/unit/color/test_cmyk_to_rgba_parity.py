import math
import unittest

from src.color import cmyk_to_rgba


class TestCmykToRgbaParity(unittest.TestCase):
    def test_rounds_half_up_like_typescript(self):
        self.assertEqual(
            cmyk_to_rgba(60, 20, 10, 25, 0.5),
            {"r": 77, "g": 153, "b": 172, "a": 0.5},
        )
        self.assertEqual(
            cmyk_to_rgba(0, 100, 25, 90, 0.25),
            {"r": 26, "g": 0, "b": 19, "a": 0.25},
        )
        self.assertEqual(
            cmyk_to_rgba(0, 70, 20, 30),
            {"r": 179, "g": 54, "b": 143, "a": 1},
        )

    def test_clamps_out_of_range_values(self):
        self.assertEqual(
            cmyk_to_rgba(150, -20, 50, 0, 2),
            {"r": 0, "g": 255, "b": 128, "a": 1},
        )
        self.assertEqual(cmyk_to_rgba(0, 0, 0, 0, -1)["a"], 0)

    def test_propagates_nan(self):
        result = cmyk_to_rgba(float("nan"), 0, 0, 0)
        self.assertTrue(math.isnan(result["r"]))
        self.assertEqual(result["g"], 255)
        self.assertEqual(result["b"], 255)
        self.assertEqual(result["a"], 1)


if __name__ == "__main__":
    unittest.main()
