import unittest

from src.ip import cidr_to_long


class TestCidrToLong(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(cidr_to_long(24), 4294967040)
        self.assertEqual(cidr_to_long(32), 4294967295)
        self.assertEqual(cidr_to_long(0), 0)

    def test_common_cidrs(self):
        self.assertEqual(cidr_to_long(8), 4278190080)
        self.assertEqual(cidr_to_long(16), 4294901760)

    def test_invalid_cidr(self):
        with self.assertRaises(ValueError):
            cidr_to_long(-1)
        with self.assertRaises(ValueError):
            cidr_to_long(33)

    def test_docstring_example(self):
        self.assertEqual(cidr_to_long(24), 4294967040)
        self.assertEqual(cidr_to_long(32), 4294967295)
        self.assertEqual(cidr_to_long(0), 0)


if __name__ == "__main__":
    unittest.main()
