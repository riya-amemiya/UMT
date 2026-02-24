
import unittest
from src.math import clamp

class TestClamp(unittest.TestCase):
    def test_should_return_the_value_when_within_range(self):
        self.assertEqual(clamp(5, 0, 10), 5)

    def test_should_return_the_min_when_value_is_below_range(self):
        self.assertEqual(clamp(-3, 0, 10), 0)

    def test_should_return_the_max_when_value_is_above_range(self):
        self.assertEqual(clamp(15, 0, 10), 10)

    def test_should_return_min_when_value_equals_min(self):
        self.assertEqual(clamp(0, 0, 10), 0)

    def test_should_return_max_when_value_equals_max(self):
        self.assertEqual(clamp(10, 0, 10), 10)

    def test_should_handle_negative_ranges(self):
        self.assertEqual(clamp(-5, -10, -1), -5)
        self.assertEqual(clamp(0, -10, -1), -1)
        self.assertEqual(clamp(-20, -10, -1), -10)

    def test_should_handle_floating_point_values(self):
        self.assertEqual(clamp(0.5, 0, 1), 0.5)
        self.assertEqual(clamp(1.5, 0, 1), 1)
        self.assertEqual(clamp(-0.5, 0, 1), 0)

    def test_should_handle_min_equal_to_max(self):
        self.assertEqual(clamp(5, 3, 3), 3)
        self.assertEqual(clamp(1, 3, 3), 3)
