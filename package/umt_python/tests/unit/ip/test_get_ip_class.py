import unittest

from src.ip import get_ip_class


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

    def test_invalid_format_wrong_octets(self):
        self.assertEqual(get_ip_class("192.168.1"), "")
        self.assertEqual(get_ip_class("192.168"), "")
        self.assertEqual(get_ip_class("192.168.1.1.1"), "")

    def test_invalid_format_non_numeric(self):
        self.assertEqual(get_ip_class("abc.168.1.1"), "")
        self.assertEqual(get_ip_class("xyz.0.0.1"), "")
        self.assertEqual(get_ip_class("hello.168.1.1"), "")

    def test_invalid_format_out_of_range(self):
        self.assertEqual(get_ip_class("256.168.1.1"), "")
        self.assertEqual(get_ip_class("-1.168.1.1"), "")

    def test_docstring_example(self):
        self.assertEqual(get_ip_class("10.0.0.1"), "A")
        self.assertEqual(get_ip_class("172.16.0.1"), "B")
        self.assertEqual(get_ip_class("192.168.1.1"), "C")


if __name__ == "__main__":
    unittest.main()
