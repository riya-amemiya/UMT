import unittest

from src.validate import is_not_empty


class TestIsNotEmptyList(unittest.TestCase):
    def test_returns_true_for_a_non_empty_list(self):
        self.assertTrue(is_not_empty([1, 2, 3]))
        self.assertTrue(is_not_empty(["a"]))
        self.assertTrue(is_not_empty([{"a": 1}]))

    def test_returns_false_for_an_empty_list(self):
        self.assertFalse(is_not_empty([]))

    def test_returns_true_for_a_non_empty_dictionary(self):
        self.assertTrue(is_not_empty({"a": 1}))

    def test_returns_false_for_an_empty_dictionary(self):
        self.assertFalse(is_not_empty({}))

    def test_returns_false_for_sets_regardless_of_content(self):
        self.assertFalse(is_not_empty({1, 2, 3}))
        self.assertFalse(is_not_empty(set()))

    def test_returns_false_for_other_values(self):
        self.assertTrue(is_not_empty("hello"))
        self.assertFalse(is_not_empty(123))
        with self.assertRaises(TypeError):
            is_not_empty(None)


if __name__ == "__main__":
    unittest.main()
