import unittest

from src.string import uncapitalize


class TestUncapitalize(unittest.TestCase):
    def test_lowercases_first_letter(self):
        self.assertEqual(uncapitalize("Hello"), "hello")

    def test_preserves_rest_of_string(self):
        self.assertEqual(uncapitalize("ÉCLAIR"), "éCLAIR")

    def test_empty_input(self):
        self.assertEqual(uncapitalize(""), "")

    def test_surrogate_pair_first_character(self):
        text = "\U0001f600ABC"
        self.assertEqual(uncapitalize(text), text)


if __name__ == "__main__":
    unittest.main()
