import unittest

from src.string import encode_base64_url


class TestEncodeBase64Url(unittest.TestCase):
    def test_encodes_without_padding(self):
        self.assertEqual(encode_base64_url("Hello World"), "SGVsbG8gV29ybGQ")
        self.assertEqual(encode_base64_url("f"), "Zg")
        self.assertEqual(encode_base64_url("fo"), "Zm8")
        self.assertEqual(encode_base64_url("foo"), "Zm9v")

    def test_uses_url_safe_alphabet(self):
        self.assertEqual(encode_base64_url("\xff\xfe"), "w7_Dvg")
        self.assertEqual(encode_base64_url("\U0001f600?>"), "8J-YgD8-")

    def test_encodes_an_empty_string(self):
        self.assertEqual(encode_base64_url(""), "")


if __name__ == "__main__":
    unittest.main()
