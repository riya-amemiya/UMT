import unittest

from src.date import birthday


class TestBirthday(unittest.TestCase):
    def test_basic_cases(self):
        age = birthday(2000, 1, 1)
        self.assertIsInstance(age, int)
        self.assertGreaterEqual(age, 0)

    def test_edge_cases(self):
        future_age = birthday(2100, 1, 1)
        self.assertEqual(future_age, 0)

    def test_returns_integer(self):
        age = birthday(1990, 6, 15)
        self.assertIsInstance(age, int)


if __name__ == "__main__":
    unittest.main()
