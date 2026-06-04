import unittest

from src.string import mask


class TestMask(unittest.TestCase):
    def test_masks_middle_with_defaults(self):
        self.assertEqual(mask("secret"), "s****t")

    def test_masks_with_custom_start_and_end(self):
        self.assertEqual(mask("1234567890", start=2, end=4), "12****7890")

    def test_uses_custom_mask_character(self):
        self.assertEqual(mask("secret", char="#"), "s####t")

    def test_unchanged_when_start_end_equal_length(self):
        self.assertEqual(mask("abc", start=2, end=1), "abc")

    def test_unchanged_when_start_end_exceeds_length(self):
        self.assertEqual(mask("abc", start=5, end=5), "abc")

    def test_surrogate_pairs_as_single_graphemes(self):
        text = "\U0001f600abcde\U0001f600"
        expected = "\U0001f600*****\U0001f600"
        self.assertEqual(mask(text, start=1, end=1), expected)

    def test_empty_string(self):
        self.assertEqual(mask(""), "")


if __name__ == "__main__":
    unittest.main()
