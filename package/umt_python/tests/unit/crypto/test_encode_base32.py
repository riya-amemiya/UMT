import unittest

from src.crypto import encode_base32


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


if __name__ == "__main__":
    unittest.main()
