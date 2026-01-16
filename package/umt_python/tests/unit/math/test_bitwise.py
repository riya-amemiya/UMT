import unittest

from src.math import bitwise


class TestBitwise(unittest.TestCase):
    def test_left_rotation(self):
        result = bitwise(0x12345678, 8)
        self.assertEqual(hex(result), "0x34567812")

    def test_right_rotation(self):
        result = bitwise(0x12345678, 8, "right")
        self.assertEqual(hex(result), "0x78123456")

    def test_docstring_example(self):
        self.assertEqual(hex(bitwise(0x12345678, 8)), "0x34567812")
        self.assertEqual(hex(bitwise(0x12345678, 8, "right")), "0x78123456")


if __name__ == "__main__":
    unittest.main()
