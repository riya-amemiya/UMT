import unittest

from src.ip import ip_to_long, long_to_ip


class TestLongToIp(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(long_to_ip(3232235777), "192.168.1.1")
        self.assertEqual(long_to_ip(0), "0.0.0.0")
        self.assertEqual(long_to_ip(4294967295), "255.255.255.255")

    def test_common_addresses(self):
        self.assertEqual(long_to_ip(0xC0A80001), "192.168.0.1")
        self.assertEqual(long_to_ip(0x80000001), "128.0.0.1")
        self.assertEqual(long_to_ip(0x0A000001), "10.0.0.1")
        self.assertEqual(long_to_ip(0xAC100001), "172.16.0.1")
        self.assertEqual(long_to_ip(0x7F000001), "127.0.0.1")
        self.assertEqual(long_to_ip(0x01020304), "1.2.3.4")

    def test_float_integer(self):
        self.assertEqual(long_to_ip(3232235777.0), "192.168.1.1")

    def test_round_trip_with_ip_to_long(self):
        for ip in (
            "0.0.0.0",
            "1.2.3.4",
            "10.0.0.1",
            "127.0.0.1",
            "192.168.1.1",
            "255.255.255.255",
        ):
            self.assertEqual(long_to_ip(ip_to_long(ip)), ip)

    def test_invalid_input(self):
        with self.assertRaises(ValueError):
            long_to_ip(-1)
        with self.assertRaises(ValueError):
            long_to_ip(4294967296)
        with self.assertRaises(ValueError):
            long_to_ip(float("inf"))
        with self.assertRaises(ValueError):
            long_to_ip(float("nan"))
        with self.assertRaises(ValueError):
            long_to_ip(1.5)

    def test_docstring_example(self):
        self.assertEqual(long_to_ip(3232235777), "192.168.1.1")
        self.assertEqual(long_to_ip(0), "0.0.0.0")


if __name__ == "__main__":
    unittest.main()
