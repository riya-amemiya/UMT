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

    def test_invalid_lightness(self):
        """Test that invalid lightness values raise ValueError."""
        with self.assertRaises(ValueError) as context:
            hsla_to_rgba(180, 50, -1)
        self.assertIn("Lightness", str(context.exception))

        with self.assertRaises(ValueError) as context:
            hsla_to_rgba(180, 50, 101)
        self.assertIn("Lightness", str(context.exception))

    def test_invalid_alpha(self):
        """Test that invalid alpha values raise ValueError."""
        with self.assertRaises(ValueError) as context:
            hsla_to_rgba(180, 50, 50, -0.1)
        self.assertIn("Alpha", str(context.exception))

        with self.assertRaises(ValueError) as context:
            hsla_to_rgba(180, 50, 50, 1.1)
        self.assertIn("Alpha", str(context.exception))

    def test_hue_to_rgb_t_greater_than_one(self):
        """Test hue_to_rgb branch where t > 1 (t_adjusted = t - 1)."""

        result = hsla_to_rgba(0, 100, 50)
        self.assertAlmostEqual(result["r"], 255, places=0)
        self.assertAlmostEqual(result["g"], 0, places=0)
        self.assertAlmostEqual(result["b"], 0, places=0)

    def test_hue_to_rgb_between_half_and_two_thirds(self):
        """Test hue_to_rgb branch where 1/2 <= t_adjusted < 2/3."""

        result = hsla_to_rgba(240, 100, 50)
        self.assertAlmostEqual(result["r"], 0, places=0)
        self.assertAlmostEqual(result["g"], 0, places=0)
        self.assertAlmostEqual(result["b"], 255, places=0)

        result = hsla_to_rgba(180, 100, 50)
        self.assertAlmostEqual(result["r"], 0, places=0)
        self.assertAlmostEqual(result["g"], 255, places=0)
        self.assertAlmostEqual(result["b"], 255, places=0)

    def test_various_hue_values(self):
        """Test various hue values to cover different hue_to_rgb branches."""

        result = hsla_to_rgba(60, 100, 50)
        self.assertAlmostEqual(result["r"], 255, places=0)
        self.assertAlmostEqual(result["g"], 255, places=0)
        self.assertAlmostEqual(result["b"], 0, places=0)

        result = hsla_to_rgba(300, 100, 50)
        self.assertAlmostEqual(result["r"], 255, places=0)
        self.assertAlmostEqual(result["g"], 0, places=0)
        self.assertAlmostEqual(result["b"], 255, places=0)

    def test_lightness_above_half(self):
        """Test with lightness > 0.5 to cover the else branch of q calculation."""
        result = hsla_to_rgba(120, 50, 75)

        self.assertTrue(result["g"] > result["r"])
        self.assertTrue(result["g"] > result["b"])


if __name__ == "__main__":
    unittest.main()
