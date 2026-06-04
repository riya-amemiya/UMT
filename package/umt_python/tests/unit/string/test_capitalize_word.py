import unittest

from src.string import capitalize_word


class TestCapitalizeWord(unittest.TestCase):
    def test_uppercases_first_and_lowercases_rest(self):
        self.assertEqual(capitalize_word("hello"), "Hello")

    def test_normalizes_mixed_case(self):
        self.assertEqual(capitalize_word("hELLO"), "Hello")

    def test_empty_input(self):
        self.assertEqual(capitalize_word(""), "")

    def test_single_character(self):
        self.assertEqual(capitalize_word("a"), "A")


if __name__ == "__main__":
    unittest.main()
