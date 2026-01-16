import unittest

from src.ip import long_to_ip


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


if __name__ == "__main__":
    unittest.main()
