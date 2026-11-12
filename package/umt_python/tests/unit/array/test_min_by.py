import math
import unittest

from src.array import min_by


class TestMinBy(unittest.TestCase):
    def test_returns_smallest(self):
        self.assertEqual(
            min_by([{"n": 3}, {"n": 1}, {"n": 2}], lambda item: item["n"]), {"n": 1}
        )

    def test_returns_none_for_empty(self):
        self.assertIsNone(min_by([], lambda n: n))

    def test_single_element(self):
        self.assertEqual(min_by([7], lambda n: n), 7)

    def test_first_wins_ties(self):
        first = {"n": 1, "id": "first"}
        second = {"n": 1, "id": "second"}
        self.assertIs(
            min_by([{"n": 2, "id": "x"}, first, second], lambda item: item["n"]),
            first,
        )

    def test_compares_strings(self):
        self.assertEqual(min_by(["pear", "apple", "fig"], lambda s: s), "apple")

    def test_nan_never_replaces_minimum(self):
        self.assertEqual(min_by([2.0, math.nan, 1.0], lambda n: n), 1.0)
        result = min_by([math.nan, 1.0], lambda n: n)
        self.assertTrue(result is not None and math.isnan(result))


if __name__ == "__main__":
    unittest.main()
