import math
import unittest

from src.math import deg_to_rad
from src.math.division import division
from src.math.multiplication import multiplication


class TestDegToRad(unittest.TestCase):
    def test_basic_cases(self):
        self.assertAlmostEqual(deg_to_rad(180), math.pi)
        self.assertAlmostEqual(deg_to_rad(90), math.pi / 2)
        self.assertAlmostEqual(deg_to_rad(0), 0)
        self.assertAlmostEqual(deg_to_rad(270), 3 * math.pi / 2)

    def test_edge_cases(self):
        self.assertAlmostEqual(deg_to_rad(360), 2 * math.pi)
        self.assertAlmostEqual(deg_to_rad(-90), -math.pi / 2)
        self.assertAlmostEqual(deg_to_rad(-180), -math.pi)
        self.assertAlmostEqual(deg_to_rad(450), 5 * math.pi / 2)
        self.assertAlmostEqual(deg_to_rad(720), 4 * math.pi)

    def test_floating_point_angles(self):
        self.assertAlmostEqual(deg_to_rad(45.5), math.pi / 4 + math.pi / 360)
        self.assertAlmostEqual(deg_to_rad(60.7), math.pi * 60.7 / 180)
        self.assertAlmostEqual(deg_to_rad(120.2), math.pi * 120.2 / 180)

    def test_matches_unhoisted_formula(self):
        for angle in (0, 45.5, 90, 180, 270, 360, -90, 450, 720):
            self.assertEqual(
                deg_to_rad(angle),
                multiplication(angle, division(math.pi, 180)),
            )

    def test_docstring_example(self):
        self.assertAlmostEqual(round(deg_to_rad(180), 10), 3.1415926536)


if __name__ == "__main__":
    unittest.main()
