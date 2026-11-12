import unittest

from src.ip import parse_cidr


class TestParseCidr(unittest.TestCase):
    def test_valid_cidr(self):
        for cidr, network_address, prefix_length in [
            ("192.168.0.0/24", "192.168.0.0", 24),
            ("192.168.1.130/25", "192.168.1.128", 25),
            ("10.1.2.3/8", "10.0.0.0", 8),
            ("172.31.255.255/12", "172.16.0.0", 12),
            ("255.255.255.255/1", "128.0.0.0", 1),
            ("10.0.0.1/0", "0.0.0.0", 0),
            ("10.0.0.1/32", "10.0.0.1", 32),
        ]:
            with self.subTest(cidr=cidr):
                self.assertEqual(
                    parse_cidr(cidr),
                    {
                        "network_address": network_address,
                        "prefix_length": prefix_length,
                    },
                )

    def test_invalid_prefix(self):
        for cidr in [
            "192.168.0.0",
            "192.168.0.0/",
            "192.168.0.0/33",
            "192.168.0.0/-1",
            "192.168.0.0/08",
            "192.168.0.0/+8",
            "192.168.0.0/ 8",
            "192.168.0.0/8\n",
            "192.168.0.0/24/8",
            "192.168.0.0/\N{ARABIC-INDIC DIGIT TWO}",
        ]:
            with self.subTest(cidr=cidr), self.assertRaises(ValueError):
                parse_cidr(cidr)

    def test_invalid_address(self):
        for cidr in ["/24", "192.168.0/24", "256.0.0.0/8", "192.168.00.0/24"]:
            with self.subTest(cidr=cidr), self.assertRaises(ValueError):
                parse_cidr(cidr)


if __name__ == "__main__":
    unittest.main()
