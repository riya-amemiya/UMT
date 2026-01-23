import unittest

from src.ip import get_network_address


class TestGetNetworkAddress(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(
            get_network_address("192.168.1.100", "255.255.255.0"), 3232235776
        )
        self.assertEqual(get_network_address("10.0.1.5", "255.0.0.0"), 167772160)

    def test_empty_ip(self):
        with self.assertRaises(ValueError) as context:
            get_network_address("", "255.255.255.0")
        self.assertIn("IP address is required", str(context.exception))

    def test_empty_subnet_mask(self):
        with self.assertRaises(ValueError) as context:
            get_network_address("192.168.1.100", "")
        self.assertIn("Subnet mask is required", str(context.exception))

    def test_invalid_ip_format_wrong_octets(self):
        with self.assertRaises(TypeError) as context:
            get_network_address("192.168.1", "255.255.255.0")
        self.assertIn("Invalid IP address or subnet mask", str(context.exception))

    def test_invalid_ip_format_non_numeric(self):
        with self.assertRaises(TypeError) as context:
            get_network_address("192.abc.1.1", "255.255.255.0")
        self.assertIn("Invalid IP address or subnet mask", str(context.exception))

    def test_invalid_ip_format_out_of_range(self):
        with self.assertRaises(TypeError) as context:
            get_network_address("192.256.1.1", "255.255.255.0")
        self.assertIn("Invalid IP address or subnet mask", str(context.exception))

        with self.assertRaises(TypeError) as context:
            get_network_address("192.-1.1.1", "255.255.255.0")
        self.assertIn("Invalid IP address or subnet mask", str(context.exception))

    def test_invalid_mask_format_wrong_octets(self):
        with self.assertRaises(TypeError) as context:
            get_network_address("192.168.1.1", "255.255.255")
        self.assertIn("Invalid IP address or subnet mask", str(context.exception))

    def test_invalid_mask_format_non_numeric(self):
        with self.assertRaises(TypeError) as context:
            get_network_address("192.168.1.1", "255.abc.255.0")
        self.assertIn("Invalid IP address or subnet mask", str(context.exception))

    def test_invalid_mask_format_out_of_range(self):
        with self.assertRaises(TypeError) as context:
            get_network_address("192.168.1.1", "256.255.255.0")
        self.assertIn("Invalid IP address or subnet mask", str(context.exception))

    def test_invalid_subnet_mask_not_consecutive(self):
        with self.assertRaises(TypeError) as context:
            get_network_address("192.168.1.1", "255.0.255.0")
        self.assertIn("Invalid IP address or subnet mask", str(context.exception))

    def test_docstring_example(self):
        self.assertEqual(
            get_network_address("192.168.1.100", "255.255.255.0"), 3232235776
        )


if __name__ == "__main__":
    unittest.main()
