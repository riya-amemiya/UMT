import time
import unittest

from src.math import uuidv7


class TestUuidv7Parity(unittest.TestCase):
    def test_timestamp_prefix_matches_current_time(self):
        before = time.time_ns() // 1_000_000
        result = uuidv7()
        after = time.time_ns() // 1_000_000
        timestamp = int(result[:8] + result[9:13], 16)
        self.assertGreaterEqual(timestamp, before)
        self.assertLessEqual(timestamp, after)

    def test_length_and_separators(self):
        result = uuidv7()
        self.assertEqual(len(result), 36)
        self.assertEqual([result[index] for index in (8, 13, 18, 23)], ["-"] * 4)

    def test_random_bits_vary(self):
        self.assertEqual(len({uuidv7()[14:] for _ in range(50)}), 50)


if __name__ == "__main__":
    unittest.main()
