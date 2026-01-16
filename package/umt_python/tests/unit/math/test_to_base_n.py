import unittest

from src.math import to_base_n


class TestToBaseN(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(to_base_n(10, 2), "1010")
        self.assertEqual(to_base_n(255, 16), "ff")

    def test_edge_cases(self):
        self.assertEqual(to_base_n(0, 2), "0")


if __name__ == "__main__":
    unittest.main()
