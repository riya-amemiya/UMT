import math
import unittest

from src.math import rad_to_deg
from src.math.division import division


class TestRadToDeg(unittest.TestCase):
    def test_basic_cases(self):
        self.assertAlmostEqual(rad_to_deg(math.pi), 180)
        self.assertAlmostEqual(rad_to_deg(math.pi / 2), 90)
        self.assertAlmostEqual(rad_to_deg(0), 0)
        self.assertAlmostEqual(rad_to_deg(3 * math.pi / 2), 270)

    def test_edge_cases(self):
        self.assertAlmostEqual(rad_to_deg(2 * math.pi), 360)
        self.assertAlmostEqual(rad_to_deg(-math.pi / 2), -90)
        self.assertAlmostEqual(rad_to_deg(-math.pi), -180)
        self.assertAlmostEqual(rad_to_deg(5 * math.pi / 2), 450)
        self.assertAlmostEqual(rad_to_deg(4 * math.pi), 720)

    def test_floating_point_angles(self):
        self.assertAlmostEqual(rad_to_deg(math.pi / 4), 45)
        self.assertAlmostEqual(rad_to_deg(math.pi * 60.7 / 180), 60.7)
        self.assertAlmostEqual(rad_to_deg(math.pi * 120.2 / 180), 120.2)

    def test_matches_unhoisted_formula(self):
        for angle in (
            0,
            math.pi / 4,
            math.pi / 2,
            math.pi,
            3 * math.pi / 2,
            2 * math.pi,
            -math.pi / 2,
            5 * math.pi / 2,
            4 * math.pi,
        ):
            self.assertEqual(
                rad_to_deg(angle),
                division(angle, division(math.pi, 180)),
            )

    def test_docstring_example(self):
        self.assertEqual(rad_to_deg(math.pi), 180.0)


if __name__ == "__main__":
    unittest.main()
