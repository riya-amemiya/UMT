import unittest

from src.crypto import encode_hex


class TestEncodeHex(unittest.TestCase):
    def test_encodes_a_string_as_utf8(self):
        self.assertEqual(encode_hex("Hello"), "48656c6c6f")

    def test_encodes_multi_byte_characters(self):
        self.assertEqual(encode_hex("\N{HIRAGANA LETTER A}"), "e38182")
        self.assertEqual(encode_hex("\U0001f600"), "f09f9880")

    def test_encodes_bytes_with_lowercase_digits_and_zero_padding(self):
        self.assertEqual(encode_hex(bytes([0, 1, 15, 16, 171, 255])), "00010f10abff")

    def test_encodes_empty_input(self):
        self.assertEqual(encode_hex(""), "")
        self.assertEqual(encode_hex(b""), "")


if __name__ == "__main__":
    unittest.main()
