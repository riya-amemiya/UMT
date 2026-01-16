import math
import unittest

from src.math import deg_to_rad


class TestDegToRad(unittest.TestCase):
    def test_basic_cases(self):
        self.assertAlmostEqual(deg_to_rad(180), math.pi)
        self.assertAlmostEqual(deg_to_rad(90), math.pi / 2)
        self.assertAlmostEqual(deg_to_rad(0), 0)

    def test_edge_cases(self):
        self.assertAlmostEqual(deg_to_rad(360), 2 * math.pi)
        self.assertAlmostEqual(deg_to_rad(-90), -math.pi / 2)

    def test_docstring_example(self):
        self.assertAlmostEqual(round(deg_to_rad(180), 10), 3.1415926536)


if __name__ == "__main__":
    unittest.main()
