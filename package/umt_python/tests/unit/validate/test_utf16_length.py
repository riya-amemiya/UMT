import unittest

from src.validate.utf16_length import utf16_length


class TestUtf16Length(unittest.TestCase):
    def test_counts_ascii_strings(self):
        self.assertEqual(utf16_length(""), 0)
        self.assertEqual(utf16_length("abc"), 3)

    def test_counts_basic_multilingual_plane_characters_once(self):
        self.assertEqual(utf16_length(chr(0x3042) * 2), 2)
        self.assertEqual(utf16_length("a" + chr(0xE9)), 2)

    def test_counts_astral_characters_twice(self):
        self.assertEqual(utf16_length(chr(0x1F600)), 2)
        self.assertEqual(utf16_length("a" + chr(0x1F600) + "b"), 4)

    def test_counts_lone_surrogates_once(self):
        self.assertEqual(utf16_length(chr(0xD800)), 1)


if __name__ == "__main__":
    unittest.main()
