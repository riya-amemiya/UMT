import unittest

from src.date import day_of_week


class TestDayOfWeek(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(day_of_week(2000, 1, 1), 5)
        self.assertEqual(day_of_week(2025, 1, 1), 2)

    def test_edge_cases(self):
        self.assertEqual(day_of_week(2024, 2, 29), 3)

    def test_docstring_example(self):
        self.assertEqual(day_of_week(2000, 1, 1), 5)


if __name__ == "__main__":
    unittest.main()
