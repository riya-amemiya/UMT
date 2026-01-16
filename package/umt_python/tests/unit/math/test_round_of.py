import unittest

from src.math import round_of


class TestRoundOf(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(round_of(1.234, 2), 1.23)
        self.assertEqual(round_of(1.235, 2), 1.24)
        self.assertEqual(round_of(-1.234, 2), -1.23)

    def test_edge_cases(self):
        self.assertEqual(round_of(1.5, 0), 2)
        self.assertEqual(round_of(1.4, 0), 1)

    def test_docstring_example(self):
        self.assertEqual(round_of(1.234, 2), 1.23)
        self.assertEqual(round_of(1.235, 2), 1.24)
        self.assertEqual(round_of(-1.234, 2), -1.23)


if __name__ == "__main__":
    unittest.main()
