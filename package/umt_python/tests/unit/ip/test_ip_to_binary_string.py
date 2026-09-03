import unittest

from src.ip import ip_to_binary_string, ip_to_long, long_to_ip


class TestIpToBinaryString(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(
            ip_to_binary_string("192.168.1.1"), "11000000101010000000000100000001"
        )
        self.assertEqual(
            ip_to_binary_string("0.0.0.0"), "00000000000000000000000000000000"
        )

    def test_common_addresses(self):
        self.assertEqual(
            ip_to_binary_string("192.168.0.1"), "11000000101010000000000000000001"
        )
        self.assertEqual(
            ip_to_binary_string("255.255.255.255"), "11111111111111111111111111111111"
        )
        self.assertEqual(
            ip_to_binary_string("1.2.3.4"), "00000001000000100000001100000100"
        )
        self.assertEqual(
            ip_to_binary_string("10.0.0.1"), "00001010000000000000000000000001"
        )
        self.assertEqual(
            ip_to_binary_string("172.16.0.1"), "10101100000100000000000000000001"
        )
        self.assertEqual(
            ip_to_binary_string("127.0.0.1"), "01111111000000000000000000000001"
        )
        self.assertEqual(
            ip_to_binary_string("169.254.0.1"), "10101001111111100000000000000001"
        )

    def test_boundary_values(self):
        self.assertEqual(
            ip_to_binary_string("1.1.1.1"), "00000001000000010000000100000001"
        )
        self.assertEqual(
            ip_to_binary_string("128.0.0.0"), "10000000000000000000000000000000"
        )
        self.assertEqual(
            ip_to_binary_string("0.255.0.255"), "00000000111111110000000011111111"
        )

    def test_invalid_addresses(self):
        with self.assertRaises(ValueError):
            ip_to_binary_string("")
        with self.assertRaises(ValueError):
            ip_to_binary_string("invalid")

    def test_empty_string_message(self):
        with self.assertRaises(ValueError) as context:
            ip_to_binary_string("")
        self.assertIn("IP address is required", str(context.exception))

    def test_leading_zeros_invalid(self):
        with self.assertRaises(ValueError) as context:
            ip_to_binary_string("192.168.01.1")
        self.assertIn("Invalid IP address format", str(context.exception))

        with self.assertRaises(ValueError) as context:
            ip_to_binary_string("192.168.001.1")
        self.assertIn("Invalid IP address format", str(context.exception))

        with self.assertRaises(ValueError) as context:
            ip_to_binary_string("192.168.1.01")
        self.assertIn("Invalid IP address format", str(context.exception))

        with self.assertRaises(ValueError) as context:
            ip_to_binary_string("010.020.030.040")
        self.assertIn("Invalid IP address format", str(context.exception))

    def test_empty_octet(self):
        with self.assertRaises(ValueError) as context:
            ip_to_binary_string("192..1.1")
        self.assertIn("Invalid IP address format", str(context.exception))

    def test_non_numeric_octet(self):
        with self.assertRaises(ValueError) as context:
            ip_to_binary_string("192.abc.1.1")
        self.assertIn("Invalid IP address format", str(context.exception))

    def test_wrong_arity(self):
        for ip in ("192.168", "192.168.1", "1.2.3.4.5", "192.168.1.1.1", "0.0.0"):
            with self.assertRaises(ValueError) as context:
                ip_to_binary_string(ip)
            self.assertIn("Invalid IP address format", str(context.exception))

    def test_out_of_range_octet(self):
        with self.assertRaises(ValueError) as context:
            ip_to_binary_string("256.1.2.3")
        self.assertIn("Invalid IP address format", str(context.exception))

        with self.assertRaises(ValueError) as context:
            ip_to_binary_string("999.999.999.999")
        self.assertIn("Invalid IP address format", str(context.exception))

    def test_matches_ip_to_long_bits(self):
        ips = (
            "192.168.0.1",
            "0.0.0.0",
            "255.255.255.255",
            "1.2.3.4",
            "10.0.0.1",
            "172.16.0.1",
            "127.0.0.1",
            "169.254.0.1",
            "8.8.8.8",
            "128.0.0.0",
            "0.255.0.255",
            "192.168.1.1",
        )
        for ip in ips:
            binary = ip_to_binary_string(ip)
            self.assertEqual(binary, f"{ip_to_long(ip):032b}")
            self.assertEqual(len(binary), 32)

    def test_roundtrip_long_to_ip(self):
        longs = (
            0,
            1,
            0x7F000001,
            0xC0A80001,
            0xFFFFFFFF,
            0x01020304,
            0x0A000001,
            0xAC100001,
            0x80000000,
            0x00FF00FF,
        )
        for long in longs:
            ip = long_to_ip(long)
            self.assertEqual(ip_to_binary_string(ip), f"{long:032b}")
            self.assertEqual(ip_to_long(ip), long)

    def test_docstring_example(self):
        self.assertEqual(
            ip_to_binary_string("192.168.1.1"), "11000000101010000000000100000001"
        )
        self.assertEqual(
            ip_to_binary_string("0.0.0.0"), "00000000000000000000000000000000"
        )


if __name__ == "__main__":
    unittest.main()
