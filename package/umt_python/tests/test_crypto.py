import unittest

from src.crypto import (
    decode_base32,
    decode_base32_to_string,
    decode_base58,
    decode_base58_to_string,
    encode_base32,
    encode_base58,
)


class TestEncodeBase32(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(encode_base32("Hello"), "JBSWY3DP")
        self.assertEqual(encode_base32("f"), "MY======")
        self.assertEqual(encode_base32("fo"), "MZXQ====")
        self.assertEqual(encode_base32("foo"), "MZXW6===")
        self.assertEqual(encode_base32("foob"), "MZXW6YQ=")
        self.assertEqual(encode_base32("fooba"), "MZXW6YTB")
        self.assertEqual(encode_base32("foobar"), "MZXW6YTBOI======")

    def test_empty_string(self):
        self.assertEqual(encode_base32(""), "")

    def test_bytes_input(self):
        self.assertEqual(encode_base32(b"Hello"), "JBSWY3DP")

    def test_unicode(self):
        result = encode_base32("こんにちは")
        self.assertIsInstance(result, str)

    def test_docstring_example(self):
        self.assertEqual(encode_base32("Hello"), "JBSWY3DP")


class TestDecodeBase32(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(decode_base32("JBSWY3DP"), b"Hello")
        self.assertEqual(decode_base32("MY======"), b"f")
        self.assertEqual(decode_base32("MZXQ===="), b"fo")
        self.assertEqual(decode_base32("MZXW6==="), b"foo")

    def test_without_padding(self):
        self.assertEqual(decode_base32("JBSWY3DP"), b"Hello")

    def test_invalid_characters(self):
        with self.assertRaises(ValueError):
            decode_base32("invalid!@#")

    def test_docstring_example(self):
        self.assertEqual(decode_base32("JBSWY3DP"), b"Hello")


class TestDecodeBase32ToString(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(decode_base32_to_string("JBSWY3DP"), "Hello")

    def test_docstring_example(self):
        self.assertEqual(decode_base32_to_string("JBSWY3DP"), "Hello")


class TestEncodeBase58(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(encode_base58("Hello"), "9Ajdvzr")

    def test_empty_string(self):
        self.assertEqual(encode_base58(""), "")

    def test_bytes_input(self):
        self.assertEqual(encode_base58(b"Hello"), "9Ajdvzr")

    def test_leading_zeros(self):
        result = encode_base58(b"\x00\x00Hello")
        self.assertTrue(result.startswith("11"))

    def test_docstring_example(self):
        self.assertEqual(encode_base58("Hello"), "9Ajdvzr")


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


class TestDecodeBase58ToString(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(decode_base58_to_string("9Ajdvzr"), "Hello")

    def test_docstring_example(self):
        self.assertEqual(decode_base58_to_string("9Ajdvzr"), "Hello")


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
