import unittest

from src.crypto import decode_base32


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


if __name__ == "__main__":
    unittest.main()
