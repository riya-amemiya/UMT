import unittest

from src.crypto import decode_hex, encode_hex


class TestDecodeHex(unittest.TestCase):
    def test_decodes_lowercase_hex(self):
        self.assertEqual(decode_hex("48656c6c6f"), b"Hello")

    def test_decodes_uppercase_hex(self):
        self.assertEqual(decode_hex("00010F10ABFF"), bytes([0, 1, 15, 16, 171, 255]))

    def test_decodes_an_empty_string(self):
        self.assertEqual(decode_hex(""), b"")

    def test_round_trips_with_encode_hex(self):
        data = bytes(range(256))
        self.assertEqual(decode_hex(encode_hex(data)), data)

    def test_rejects_invalid_input(self):
        for value in [
            "0",
            "abc",
            "zz",
            "0g",
            "00 ff",
            " 00",
            "00\n",
            "\N{FULLWIDTH DIGIT ZERO}\N{FULLWIDTH DIGIT ZERO}",
        ]:
            with self.subTest(value=value), self.assertRaises(ValueError):
                decode_hex(value)


if __name__ == "__main__":
    unittest.main()
