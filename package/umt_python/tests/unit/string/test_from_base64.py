import unittest

from src import from_base64


class TestFromBase64(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(from_base64("SGVsbG8gV29ybGQ="), "Hello World")
        self.assertEqual(from_base64("UHl0aG9u"), "Python")
        self.assertEqual(from_base64("MTIzNDU="), "12345")

    def test_edge_cases(self):
        self.assertEqual(from_base64(""), "")
        with self.assertRaises(ValueError):
            from_base64("Invalid Base64")
        with self.assertRaises(ValueError):
            from_base64("SGVsbG8gV29ybGQ")

    def test_special_characters(self):
        self.assertEqual(from_base64("IUAjJCVeJiooKV8r"), "!@#$%^&*()_+")
        self.assertEqual(
            from_base64("SGVsbG8gV29ybGQhIFRoaXMgaXMgYSB0ZXN0Lg=="),
            "Hello World! This is a test.",
        )

    def test_unicode_characters(self):
        self.assertEqual(from_base64("44GT44KT44Gr44Gh44Gv5LiW55WM"), "こんにちは世界")
        self.assertEqual(from_base64("8J+Yig=="), "😊")

    def test_docstring_example(self):
        self.assertEqual(from_base64("SGVsbG8gV29ybGQ="), "Hello World")


if __name__ == "__main__":
    unittest.main()
