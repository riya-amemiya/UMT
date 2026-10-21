import unittest

from src.ip import is_in_range, is_private_ip


class TestIsPrivateIp(unittest.TestCase):
    def test_private_addresses(self):
        self.assertTrue(is_private_ip("10.0.0.1"))
        self.assertTrue(is_private_ip("10.255.255.255"))
        self.assertTrue(is_private_ip("172.16.0.1"))
        self.assertTrue(is_private_ip("172.31.255.255"))
        self.assertTrue(is_private_ip("192.168.0.1"))
        self.assertTrue(is_private_ip("192.168.255.255"))

    def test_public_addresses(self):
        self.assertFalse(is_private_ip("9.255.255.255"))
        self.assertFalse(is_private_ip("11.0.0.0"))
        self.assertFalse(is_private_ip("172.15.255.255"))
        self.assertFalse(is_private_ip("172.32.0.0"))
        self.assertFalse(is_private_ip("192.167.255.255"))
        self.assertFalse(is_private_ip("192.169.0.0"))
        self.assertFalse(is_private_ip("8.8.8.8"))
        self.assertFalse(is_private_ip("1.1.1.1"))
        self.assertFalse(is_private_ip("169.254.0.1"))
        self.assertFalse(is_private_ip("127.0.0.1"))

    def test_matches_is_in_range_rfc1918(self):
        samples = (
            "10.0.0.0",
            "10.0.0.1",
            "10.255.255.255",
            "172.16.0.0",
            "172.16.0.1",
            "172.31.255.255",
            "192.168.0.0",
            "192.168.1.1",
            "192.168.255.255",
            "9.255.255.255",
            "11.0.0.0",
            "172.15.255.255",
            "172.32.0.0",
            "192.167.255.255",
            "192.169.0.0",
            "8.8.8.8",
            "1.1.1.1",
            "169.254.0.1",
            "127.0.0.1",
            "0.0.0.0",
            "255.255.255.255",
        )
        for ip in samples:
            expected = (
                is_in_range(ip, "10.0.0.0", 8)
                or is_in_range(ip, "172.16.0.0", 12)
                or is_in_range(ip, "192.168.0.0", 16)
            )
            self.assertEqual(is_private_ip(ip), expected, ip)

    def test_invalid_addresses(self):
        with self.assertRaises(ValueError):
            is_private_ip("")

    def test_invalid_ip_format_raises_error(self):
        with self.assertRaises(ValueError) as context:
            is_private_ip("invalid.ip")
        self.assertIn("Invalid IP address", str(context.exception))

    def test_docstring_example(self):
        self.assertTrue(is_private_ip("192.168.1.1"))
        self.assertFalse(is_private_ip("8.8.8.8"))
        self.assertTrue(is_private_ip("10.0.0.1"))


if __name__ == "__main__":
    unittest.main()
