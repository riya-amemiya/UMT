import unittest

from src.crypto import encode_base58


class TestEncodeBase58Parity(unittest.TestCase):
    def test_long_binary_input(self):
        data = bytes((index * 37 + 11) & 0xFF for index in range(64))
        self.assertEqual(
            encode_base58(data),
            "DyXZZCSiBREJ8YZ5cULd7PVdKpbkhHvHJ7otbJmLJajJtRsyq1irMMqKimYeKvRmZ8Sc2qWLhKjYR4ekM8RSzkV",
        )

    def test_leading_zero_bytes(self):
        self.assertEqual(encode_base58(b"\x00\x00\x00"), "111")
        self.assertEqual(encode_base58(b"\x00\x00\x01\x02\x03"), "11Ldp")

    def test_empty_input(self):
        self.assertEqual(encode_base58(""), "")
        self.assertEqual(encode_base58(b""), "")

    def test_string_and_bytes_inputs(self):
        greeting = (
            "\N{HIRAGANA LETTER KO}\N{HIRAGANA LETTER N}\N{HIRAGANA LETTER NI}"
            "\N{HIRAGANA LETTER TI}\N{HIRAGANA LETTER HA}"
        )
        self.assertEqual(encode_base58(greeting), "7NAasPYBzpyEe5hmwr1KL")
        self.assertEqual(encode_base58(b"\xff\xfe\xfd\xfc\xfb"), "Vt9aq46")


if __name__ == "__main__":
    unittest.main()
