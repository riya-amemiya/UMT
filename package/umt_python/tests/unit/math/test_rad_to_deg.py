import math
import unittest

from src.math import rad_to_deg


class TestRadToDeg(unittest.TestCase):
    def test_basic_cases(self):
        self.assertAlmostEqual(rad_to_deg(math.pi), 180)
        self.assertAlmostEqual(rad_to_deg(math.pi / 2), 90)
        self.assertAlmostEqual(rad_to_deg(0), 0)

    def test_edge_cases(self):
        self.assertAlmostEqual(rad_to_deg(2 * math.pi), 360)
        self.assertAlmostEqual(rad_to_deg(-math.pi / 2), -90)

    def test_docstring_example(self):
        self.assertEqual(rad_to_deg(math.pi), 180.0)


if __name__ == "__main__":
    unittest.main()
