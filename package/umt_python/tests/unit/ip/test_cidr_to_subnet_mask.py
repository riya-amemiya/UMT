import unittest

from src.ip import cidr_to_subnet_mask


class TestCidrToSubnetMask(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(cidr_to_subnet_mask(24), "255.255.255.0")
        self.assertEqual(cidr_to_subnet_mask(16), "255.255.0.0")
        self.assertEqual(cidr_to_subnet_mask(8), "255.0.0.0")

    def test_edge_cases(self):
        self.assertEqual(cidr_to_subnet_mask(32), "255.255.255.255")
        self.assertEqual(cidr_to_subnet_mask(0), "0.0.0.0")

    def test_docstring_example(self):
        self.assertEqual(cidr_to_subnet_mask(24), "255.255.255.0")
        self.assertEqual(cidr_to_subnet_mask(16), "255.255.0.0")


if __name__ == "__main__":
    unittest.main()
