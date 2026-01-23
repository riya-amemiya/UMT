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

    def test_leading_zeros_invalid(self):
        with self.assertRaises(ValueError) as context:
            ip_to_binary_string("192.168.01.1")
        self.assertIn("Invalid IP address format", str(context.exception))

        with self.assertRaises(ValueError) as context:
            ip_to_binary_string("192.168.001.1")
        self.assertIn("Invalid IP address format", str(context.exception))

    def test_empty_octet(self):
        with self.assertRaises(ValueError) as context:
            ip_to_binary_string("192..1.1")
        self.assertIn("Invalid IP address format", str(context.exception))

    def test_non_numeric_octet(self):
        with self.assertRaises(ValueError) as context:
            ip_to_binary_string("192.abc.1.1")
        self.assertIn("Invalid IP address format", str(context.exception))

    def test_docstring_example(self):
        self.assertEqual(
            ip_to_binary_string("192.168.1.1"), "11000000101010000000000100000001"
        )
        self.assertEqual(
            ip_to_binary_string("0.0.0.0"), "00000000000000000000000000000000"
        )


if __name__ == "__main__":
    unittest.main()
