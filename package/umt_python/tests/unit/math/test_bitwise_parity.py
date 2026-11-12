import unittest

from src.math import bitwise


class TestBitwiseParity(unittest.TestCase):
    def test_left_rotation_into_sign_bit_is_negative(self):
        self.assertEqual(bitwise(1, 31), -2147483648)

    def test_right_rotation_into_sign_bit_is_negative(self):
        self.assertEqual(bitwise(1, 1, "right"), -2147483648)

    def test_all_bits_set_stays_minus_one(self):
        self.assertEqual(bitwise(-1, 4), -1)
        self.assertEqual(bitwise(4294967295, 3, "right"), -1)

    def test_negative_rotation_count(self):
        self.assertEqual(bitwise(1, -1), -2147483648)

    def test_full_rotation_returns_input(self):
        self.assertEqual(bitwise(1, 32), 1)
        self.assertEqual(bitwise(305419896, 0, "right"), 305419896)

    def test_unsigned_input_wraps_to_signed(self):
        self.assertEqual(bitwise(2147483648, 1), 1)


if __name__ == "__main__":
    unittest.main()
