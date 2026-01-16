import unittest

from src.ip import is_private_ip


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


if __name__ == "__main__":
    unittest.main()
