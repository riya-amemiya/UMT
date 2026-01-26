import unittest

from src.crypto import (
    decode_base32,
    decode_base58,
    encode_base32,
    encode_base58,
)


class TestRoundTrip(unittest.TestCase):
    def test_base32_roundtrip(self):
        original = "Hello World!"
        encoded = encode_base32(original)
        decoded = decode_base32(encoded).decode("utf-8")
        self.assertEqual(decoded, original)

    def test_base58_roundtrip(self):
        original = "Hello World!"
        encoded = encode_base58(original)
        decoded = decode_base58(encoded).decode("utf-8")
        self.assertEqual(decoded, original)


if __name__ == "__main__":
    unittest.main()
