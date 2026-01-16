import unittest

from src.crypto import decode_base58, encode_base58


class TestDecodeBase58(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(decode_base58("9Ajdvzr"), b"Hello")

    def test_leading_ones(self):
        result = decode_base58("11" + encode_base58(b"Hello"))
        self.assertTrue(result.startswith(b"\x00\x00"))

    def test_invalid_characters(self):
        with self.assertRaises(ValueError):
            decode_base58("0OIl")

    def test_docstring_example(self):
        self.assertEqual(decode_base58("9Ajdvzr"), b"Hello")


if __name__ == "__main__":
    unittest.main()
