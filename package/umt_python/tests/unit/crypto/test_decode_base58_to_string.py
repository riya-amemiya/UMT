import unittest

from src.crypto import decode_base58_to_string


class TestDecodeBase58ToString(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(decode_base58_to_string("9Ajdvzr"), "Hello")

    def test_docstring_example(self):
        self.assertEqual(decode_base58_to_string("9Ajdvzr"), "Hello")


if __name__ == "__main__":
    unittest.main()
