import unittest

from src.math import map_range


class TestMapRange(unittest.TestCase):
    def test_maps_between_ranges(self):
        self.assertEqual(map_range(5, 0, 10, 0, 100), 50)
        self.assertEqual(map_range(0.5, 0, 1, 10, 20), 15)
        self.assertEqual(map_range(25, 0, 100, -1, 1), -0.5)

    def test_endpoints(self):
        self.assertEqual(map_range(0, 0, 10, 100, 200), 100)
        self.assertEqual(map_range(10, 0, 10, 100, 200), 200)

    def test_inverted_output_range(self):
        self.assertEqual(map_range(2.5, 0, 10, 100, 0), 75)

    def test_no_clamping(self):
        self.assertEqual(map_range(15, 0, 10, 0, 100), 150)
        self.assertEqual(map_range(-5, 0, 10, 0, 100), -50)

    def test_empty_input_range_returns_out_min(self):
        self.assertEqual(map_range(7, 5, 5, 10, 20), 10)


if __name__ == "__main__":
    unittest.main()
