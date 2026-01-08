import unittest

from src.ip import (
    cidr_to_long,
    cidr_to_subnet_mask,
    get_ip_class,
    ip_to_binary_string,
    ip_to_long,
    is_in_range,
    is_private_ip,
    long_to_ip,
)


class TestIpToLong(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(ip_to_long("192.168.1.1"), 3232235777)
        self.assertEqual(ip_to_long("0.0.0.0"), 0)
        self.assertEqual(ip_to_long("255.255.255.255"), 4294967295)

    def test_common_addresses(self):
        self.assertEqual(ip_to_long("127.0.0.1"), 2130706433)
        self.assertEqual(ip_to_long("10.0.0.1"), 167772161)

    def test_invalid_addresses(self):
        with self.assertRaises(ValueError):
            ip_to_long("")
        with self.assertRaises(ValueError):
            ip_to_long("192.168")
        with self.assertRaises(ValueError):
            ip_to_long("256.1.2.3")
        with self.assertRaises(ValueError):
            ip_to_long("a.b.c.d")

    def test_docstring_example(self):
        self.assertEqual(ip_to_long("192.168.1.1"), 3232235777)
        self.assertEqual(ip_to_long("0.0.0.0"), 0)


class TestLongToIp(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(long_to_ip(3232235777), "192.168.1.1")
        self.assertEqual(long_to_ip(0), "0.0.0.0")
        self.assertEqual(long_to_ip(4294967295), "255.255.255.255")

    def test_invalid_input(self):
        with self.assertRaises(ValueError):
            long_to_ip(-1)
        with self.assertRaises(ValueError):
            long_to_ip(4294967296)
        with self.assertRaises(ValueError):
            long_to_ip(float("inf"))

    def test_docstring_example(self):
        self.assertEqual(long_to_ip(3232235777), "192.168.1.1")
        self.assertEqual(long_to_ip(0), "0.0.0.0")


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


class TestGetIpClass(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(get_ip_class("10.0.0.1"), "A")
        self.assertEqual(get_ip_class("172.16.0.1"), "B")
        self.assertEqual(get_ip_class("192.168.1.1"), "C")

    def test_class_d_and_e(self):
        self.assertEqual(get_ip_class("224.0.0.1"), "D")
        self.assertEqual(get_ip_class("240.0.0.1"), "E")

    def test_edge_cases(self):
        self.assertEqual(get_ip_class("127.0.0.1"), "A")
        self.assertEqual(get_ip_class(""), "")
        self.assertEqual(get_ip_class("0.0.0.0"), "")

    def test_docstring_example(self):
        self.assertEqual(get_ip_class("10.0.0.1"), "A")
        self.assertEqual(get_ip_class("172.16.0.1"), "B")
        self.assertEqual(get_ip_class("192.168.1.1"), "C")


class TestIsPrivateIp(unittest.TestCase):
    def test_private_addresses(self):
        self.assertTrue(is_private_ip("10.0.0.1"))
        self.assertTrue(is_private_ip("172.16.0.1"))
        self.assertTrue(is_private_ip("192.168.1.1"))

    def test_public_addresses(self):
        self.assertFalse(is_private_ip("8.8.8.8"))
        self.assertFalse(is_private_ip("1.1.1.1"))

    def test_invalid_addresses(self):
        with self.assertRaises(ValueError):
            is_private_ip("")

    def test_docstring_example(self):
        self.assertTrue(is_private_ip("192.168.1.1"))
        self.assertFalse(is_private_ip("8.8.8.8"))
        self.assertTrue(is_private_ip("10.0.0.1"))


class TestIsInRange(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_in_range("192.168.1.100", "192.168.0.0", 16))
        self.assertFalse(is_in_range("10.0.0.1", "192.168.0.0", 16))

    def test_edge_cases(self):
        self.assertTrue(is_in_range("192.168.1.1", "192.168.1.1", 32))
        self.assertTrue(is_in_range("0.0.0.0", "0.0.0.0", 0))

    def test_invalid_input(self):
        with self.assertRaises(ValueError):
            is_in_range("", "192.168.0.0", 16)
        with self.assertRaises(ValueError):
            is_in_range("192.168.1.1", "", 16)
        with self.assertRaises(ValueError):
            is_in_range("192.168.1.1", "192.168.0.0", -1)

    def test_docstring_example(self):
        self.assertTrue(is_in_range("192.168.1.100", "192.168.0.0", 16))
        self.assertFalse(is_in_range("10.0.0.1", "192.168.0.0", 16))


if __name__ == "__main__":
    unittest.main()
