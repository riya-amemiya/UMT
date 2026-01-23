import unittest

from src.ip import subnet_mask_to_cidr


class TestSubnetMaskToCidr(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(subnet_mask_to_cidr("255.255.255.0"), 24)
        self.assertEqual(subnet_mask_to_cidr("255.255.0.0"), 16)
        self.assertEqual(subnet_mask_to_cidr("255.0.0.0"), 8)

    def test_edge_cases(self):
        self.assertEqual(subnet_mask_to_cidr("0.0.0.0"), 0)
        self.assertEqual(subnet_mask_to_cidr("255.255.255.255"), 32)
        self.assertEqual(subnet_mask_to_cidr("255.255.255.128"), 25)
        self.assertEqual(subnet_mask_to_cidr("255.255.255.192"), 26)

    def test_empty_subnet_mask(self):
        with self.assertRaises(ValueError) as context:
            subnet_mask_to_cidr("")
        self.assertIn("Subnet mask is required", str(context.exception))

    def test_invalid_format_wrong_octets(self):
        with self.assertRaises(ValueError) as context:
            subnet_mask_to_cidr("255.255.255")
        self.assertIn("Invalid subnet mask format", str(context.exception))

    def test_invalid_format_too_many_octets(self):
        with self.assertRaises(ValueError) as context:
            subnet_mask_to_cidr("255.255.255.0.0")
        self.assertIn("Invalid subnet mask format", str(context.exception))

    def test_invalid_format_non_numeric(self):
        with self.assertRaises(ValueError) as context:
            subnet_mask_to_cidr("255.abc.255.0")
        self.assertIn("Invalid subnet mask format", str(context.exception))

    def test_invalid_format_out_of_range(self):
        with self.assertRaises(ValueError) as context:
            subnet_mask_to_cidr("255.256.255.0")
        self.assertIn("Invalid subnet mask format", str(context.exception))

        with self.assertRaises(ValueError) as context:
            subnet_mask_to_cidr("255.-1.255.0")
        self.assertIn("Invalid subnet mask format", str(context.exception))

    def test_invalid_subnet_mask_not_consecutive(self):
        with self.assertRaises(ValueError) as context:
            subnet_mask_to_cidr("255.0.255.0")
        self.assertIn(
            "Invalid subnet mask: must be consecutive 1s followed by 0s",
            str(context.exception),
        )

    def test_docstring_examples(self):
        self.assertEqual(subnet_mask_to_cidr("255.255.255.0"), 24)
        self.assertEqual(subnet_mask_to_cidr("255.255.0.0"), 16)


if __name__ == "__main__":
    unittest.main()
