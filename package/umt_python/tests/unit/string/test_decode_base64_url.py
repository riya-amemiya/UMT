import unittest

from src.string import decode_base64_url, encode_base64_url


class TestDecodeBase64Url(unittest.TestCase):
    def test_decodes_input_without_padding(self):
        self.assertEqual(decode_base64_url("SGVsbG8gV29ybGQ"), "Hello World")
        self.assertEqual(decode_base64_url("Zg"), "f")
        self.assertEqual(decode_base64_url("Zm8"), "fo")
        self.assertEqual(decode_base64_url("Zm9v"), "foo")

    def test_decodes_input_with_padding(self):
        self.assertEqual(decode_base64_url("SGVsbG8gV29ybGQ="), "Hello World")
        self.assertEqual(decode_base64_url("Zg=="), "f")
        self.assertEqual(decode_base64_url("Zg="), "f")

    def test_decodes_url_safe_alphabet(self):
        self.assertEqual(decode_base64_url("w7_Dvg"), "\xff\xfe")
        self.assertEqual(decode_base64_url("8J-YgD8-"), "\U0001f600?>")

    def test_decodes_an_empty_string(self):
        self.assertEqual(decode_base64_url(""), "")

    def test_round_trips_with_encode_base64_url(self):
        value = (
            "\N{HIRAGANA LETTER KO}\N{HIRAGANA LETTER N}\N{HIRAGANA LETTER NI} ~?>/+"
        )
        self.assertEqual(decode_base64_url(encode_base64_url(value)), value)

    def test_rejects_invalid_input(self):
        for value in [
            "Z",
            "Zm9vY",
            "Zg===",
            "Zm9v=",
            "Z===",
            "w7/Dvg",
            "w7+Dvg",
            "Zg ==",
            "Zg\n",
            "=",
            "Z=g",
        ]:
            with self.subTest(value=value), self.assertRaises(ValueError):
                decode_base64_url(value)

    def test_rejects_invalid_utf8(self):
        with self.assertRaises(ValueError):
            decode_base64_url("_w")


if __name__ == "__main__":
    unittest.main()
