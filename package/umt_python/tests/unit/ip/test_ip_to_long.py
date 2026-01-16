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


if __name__ == "__main__":
    unittest.main()
