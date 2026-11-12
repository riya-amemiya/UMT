import unittest

from src.ip import is_valid_ip


class TestIsValidIp(unittest.TestCase):
    def test_valid_addresses(self):
        for ip in [
            "0.0.0.0",
            "1.2.3.4",
            "10.0.0.1",
            "127.0.0.1",
            "192.168.1.1",
            "199.249.250.255",
            "255.255.255.255",
        ]:
            with self.subTest(ip=ip):
                self.assertTrue(is_valid_ip(ip))

    def test_invalid_addresses(self):
        for ip in [
            "",
            "192.168",
            "192.168.1",
            "192.168.1.1.1",
            "192..1.1",
            "192.168.1.",
            ".192.168.1",
            "256.1.2.3",
            "1.2.3.300",
            "1234.1.1.1",
            "192.168.01.1",
            "00.0.0.0",
            "a.b.c.d",
            "+1.2.3.4",
            "-1.2.3.4",
            " 1.2.3.4",
            "1.2.3.4 ",
            "1.2.3.4\n",
            "1.2.3.4/24",
            "\N{ARABIC-INDIC DIGIT ONE}.2.3.4",
        ]:
            with self.subTest(ip=ip):
                self.assertFalse(is_valid_ip(ip))


if __name__ == "__main__":
    unittest.main()
