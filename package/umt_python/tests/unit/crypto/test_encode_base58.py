import unittest

from src.crypto import encode_base58


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


if __name__ == "__main__":
    unittest.main()
