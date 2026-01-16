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

    def test_docstring_example(self):
        self.assertEqual(get_ip_class("10.0.0.1"), "A")
        self.assertEqual(get_ip_class("172.16.0.1"), "B")
        self.assertEqual(get_ip_class("192.168.1.1"), "C")


if __name__ == "__main__":
    unittest.main()
