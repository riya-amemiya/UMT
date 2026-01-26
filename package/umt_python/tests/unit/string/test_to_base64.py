import unittest

from src import to_base64


class TestToBase64(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(to_base64("Hello World"), "SGVsbG8gV29ybGQ=")
        self.assertEqual(to_base64("Python"), "UHl0aG9u")
        self.assertEqual(to_base64("12345"), "MTIzNDU=")

    def test_edge_cases(self):
        self.assertEqual(to_base64(""), "")

    def test_special_characters(self):
        self.assertEqual(to_base64("!@#$%^&*()_+"), "IUAjJCVeJiooKV8r")
        self.assertEqual(
            to_base64("Hello World! This is a test."),
            "SGVsbG8gV29ybGQhIFRoaXMgaXMgYSB0ZXN0Lg==",
        )

    def test_unicode_characters(self):
        self.assertEqual(to_base64("こんにちは世界"), "44GT44KT44Gr44Gh44Gv5LiW55WM")
        self.assertEqual(to_base64("😊"), "8J+Yig==")

    def test_docstring_example(self):
        self.assertEqual(to_base64("Hello World"), "SGVsbG8gV29ybGQ=")


if __name__ == "__main__":
    unittest.main()
