import unittest

from src.math import xoshiro256


class TestXoshiro256Parity(unittest.TestCase):
    def test_unsigned_state_matches_reference_output_and_state(self):
        state = [4294967295, 2147483648, 123456789, 987654321]
        self.assertEqual(xoshiro256(state), 0.3441991328727454)
        self.assertEqual(state, [1159829326, 2024026858, -123456790, -854182053])

    def test_negative_state_matches_reference_output_and_state(self):
        state = [-1, -2, -3, -4]
        self.assertEqual(xoshiro256(state), 0.9921874995343387)
        self.assertEqual(state, [-3, -4, -262142, 16384])

    def test_custom_range(self):
        state = [-1, -2, -3, -4]
        self.assertEqual(xoshiro256(state, 10, 20), 10 + 0.9921874995343387 * 10)


if __name__ == "__main__":
    unittest.main()
