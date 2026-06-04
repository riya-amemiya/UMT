import unittest

from src.string import swap_case


class TestSwapCase(unittest.TestCase):
    def test_swaps_case_of_each_letter(self):
        self.assertEqual(swap_case("Hello World"), "hELLO wORLD")

    def test_all_uppercase_to_lowercase(self):
        self.assertEqual(swap_case("ABC"), "abc")

    def test_all_lowercase_to_uppercase(self):
        self.assertEqual(swap_case("abc"), "ABC")

    def test_leaves_caseless_unchanged(self):
        self.assertEqual(swap_case("abc123-日本"), "ABC123-日本")

    def test_empty_string(self):
        self.assertEqual(swap_case(""), "")


if __name__ == "__main__":
    unittest.main()
