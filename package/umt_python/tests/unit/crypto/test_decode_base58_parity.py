import unittest

from src.crypto import decode_base58


class TestDecodeBase58Parity(unittest.TestCase):
    def test_long_binary_input(self):
        self.assertEqual(
            decode_base58(
                "DyXZZCSiBREJ8YZ5cULd7PVdKpbkhHvHJ7otbJmLJajJtRsyq1irMMqKimYeKvRmZ8Sc2qWLhKjYR4ekM8RSzkV"
            ),
            bytes((index * 37 + 11) & 0xFF for index in range(64)),
        )

    def test_leading_ones(self):
        self.assertEqual(decode_base58("111"), b"\x00\x00\x00")
        self.assertEqual(decode_base58("1112"), b"\x00\x00\x00\x01")
        self.assertEqual(decode_base58("11Ldp"), b"\x00\x00\x01\x02\x03")

    def test_empty_input(self):
        self.assertEqual(decode_base58(""), b"")

    def test_highest_digit(self):
        self.assertEqual(
            decode_base58("zzzzzzzzzzzz"),
            bytes([78, 144, 10, 187, 83, 230, 183, 15, 255]),
        )

    def test_invalid_characters_raise(self):
        for value in ("0", "O", "I", "l", "9Ajdvzr!", "\N{HIRAGANA LETTER A}"):
            with self.assertRaises(ValueError) as context:
                decode_base58(value)
            self.assertIn("Invalid base58 character", str(context.exception))


if __name__ == "__main__":
    unittest.main()
