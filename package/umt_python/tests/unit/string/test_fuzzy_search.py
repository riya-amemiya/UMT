import unittest

from src.string import fuzzy_search


class TestFuzzySearch(unittest.TestCase):
    def test_basic_cases(self):
        result = fuzzy_search("hello", ["hello", "world", "helo", "help"])
        self.assertEqual(len(result), 3)
        self.assertEqual(result[0]["item"], "hello")
        self.assertEqual(result[0]["score"], 1.0)

    def test_edge_cases(self):
        self.assertEqual(fuzzy_search("", ["hello", "world"]), [])
        result = fuzzy_search("xyz", ["hello", "world"], threshold=0.0)
        self.assertEqual(len(result), 2)

    def test_threshold(self):
        result = fuzzy_search("hello", ["hello", "helo", "help"], threshold=0.9)
        self.assertEqual(len(result), 1)
        self.assertEqual(result[0]["item"], "hello")

    def test_docstring_example(self):
        result = fuzzy_search("hello", ["hello", "world", "helo", "help"])
        self.assertEqual(result[0]["item"], "hello")
        self.assertEqual(result[0]["score"], 1.0)


class TestFuzzySearchDistanceBudget(unittest.TestCase):
    def test_budget_is_floored_from_threshold(self):
        result = fuzzy_search("abcdefghij", ["abcdefghiX", "abcdefghij"], 0.9)
        self.assertEqual(result, [{"item": "abcdefghij", "score": 1.0}])

    def test_length_difference_filter(self):
        result = fuzzy_search("abc", ["", "a", "abcdefgh", "ABC"], 0.3)
        self.assertEqual(
            result,
            [
                {"item": "ABC", "score": 1.0},
                {"item": "abcdefgh", "score": 0.375},
                {"item": "a", "score": 0.33333333333333337},
            ],
        )

    def test_early_exit_inside_matrix(self):
        self.assertEqual(fuzzy_search("abcdef", ["uvwxyz"], 0.5), [])

    def test_final_distance_over_budget(self):
        self.assertEqual(fuzzy_search("abcd", ["abxy"], 0.75), [])

    def test_query_longer_than_item(self):
        result = fuzzy_search("kitchen", ["kitten"], 0.5)
        self.assertEqual(result, [{"item": "kitten", "score": 0.7142857142857143}])

    def test_case_insensitive(self):
        result = fuzzy_search("HeLLo", ["hEllO"], 1)
        self.assertEqual(result, [{"item": "hEllO", "score": 1.0}])

    def test_sorted_by_score(self):
        result = fuzzy_search("kitten", ["sitting", "mitten", "kitchen"], 0.5)
        self.assertEqual(
            [entry["item"] for entry in result], ["mitten", "kitchen", "sitting"]
        )

    def test_nan_threshold_keeps_every_item(self):
        result = fuzzy_search("ab", ["xy"], float("nan"))
        self.assertEqual(result, [{"item": "xy", "score": 0.0}])

    def test_row_buffer_reused_across_items(self):
        result = fuzzy_search("abc", ["abcdefgh", "abd", "ab"], 0.3)
        self.assertEqual([entry["item"] for entry in result], ["abd", "ab", "abcdefgh"])


if __name__ == "__main__":
    unittest.main()
