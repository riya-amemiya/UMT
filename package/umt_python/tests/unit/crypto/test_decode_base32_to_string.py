import unittest

from src.crypto import decode_base32_to_string


class TestDecodeBase32ToString(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(decode_base32_to_string("JBSWY3DP"), "Hello")

    def test_docstring_example(self):
        self.assertEqual(decode_base32_to_string("JBSWY3DP"), "Hello")


if __name__ == "__main__":
    unittest.main()
