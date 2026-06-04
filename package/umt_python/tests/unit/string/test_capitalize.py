import unittest

from src.string import capitalize


class TestCapitalize(unittest.TestCase):
    def test_uppercases_first_letter(self):
        self.assertEqual(capitalize("hello"), "Hello")

    def test_preserves_rest_of_string(self):
        self.assertEqual(capitalize("hELLO"), "HELLO")

    def test_accented_first_letter(self):
        self.assertEqual(capitalize("éclair"), "Éclair")

    def test_empty_input(self):
        self.assertEqual(capitalize(""), "")

    def test_surrogate_pair_first_character(self):
        text = "\U0001f600abc"
        self.assertEqual(capitalize(text), text)


if __name__ == "__main__":
    unittest.main()
