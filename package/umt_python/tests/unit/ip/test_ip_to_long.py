import unittest

from src.ip import ip_to_long


class TestIpToLong(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(ip_to_long("192.168.1.1"), 3232235777)
        self.assertEqual(ip_to_long("0.0.0.0"), 0)
        self.assertEqual(ip_to_long("255.255.255.255"), 4294967295)

    def test_common_addresses(self):
        self.assertEqual(ip_to_long("127.0.0.1"), 2130706433)
        self.assertEqual(ip_to_long("10.0.0.1"), 167772161)
        self.assertEqual(ip_to_long("192.168.0.1"), 0xC0A80001)
        self.assertEqual(ip_to_long("128.0.0.1"), 0x80000001)
        self.assertEqual(ip_to_long("172.16.0.1"), 0xAC100001)
        self.assertEqual(ip_to_long("1.2.3.4"), 0x01020304)

    def test_invalid_addresses(self):
        with self.assertRaises(ValueError):
            ip_to_long("")
        with self.assertRaises(ValueError):
            ip_to_long("192.168")
        with self.assertRaises(ValueError):
            ip_to_long("256.1.2.3")
        with self.assertRaises(ValueError):
            ip_to_long("a.b.c.d")
        with self.assertRaises(ValueError):
            ip_to_long("192.168.01.1")
        with self.assertRaises(ValueError):
            ip_to_long("192.168.1.1.1")
        with self.assertRaises(ValueError):
            ip_to_long("192..1.1")

    def test_docstring_example(self):
        self.assertEqual(ip_to_long("192.168.1.1"), 3232235777)
        self.assertEqual(ip_to_long("0.0.0.0"), 0)


if __name__ == "__main__":
    unittest.main()
