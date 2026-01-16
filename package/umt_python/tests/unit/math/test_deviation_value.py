import unittest

from src.math import deviation_value


class TestDeviationValue(unittest.TestCase):
    def test_basic_cases(self):
        value = 70
        avg = 60
        std_dev = 10
        self.assertEqual(deviation_value(value, avg, std_dev), 60)


if __name__ == "__main__":
    unittest.main()
