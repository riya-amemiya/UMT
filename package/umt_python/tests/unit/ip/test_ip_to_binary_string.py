import unittest

from src.ip import ip_to_binary_string


class TestIpToBinaryString(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(
            ip_to_binary_string("192.168.1.1"), "11000000101010000000000100000001"
        )
        self.assertEqual(
            ip_to_binary_string("0.0.0.0"), "00000000000000000000000000000000"
        )

    def test_invalid_addresses(self):
        with self.assertRaises(ValueError):
            ip_to_binary_string("")
        with self.assertRaises(ValueError):
            ip_to_binary_string("invalid")

    def test_docstring_example(self):
        self.assertEqual(
            ip_to_binary_string("192.168.1.1"), "11000000101010000000000100000001"
        )
        self.assertEqual(
            ip_to_binary_string("0.0.0.0"), "00000000000000000000000000000000"
        )


if __name__ == "__main__":
    unittest.main()
