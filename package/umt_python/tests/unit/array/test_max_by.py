import math
import unittest

from src.array import max_by


class TestMaxBy(unittest.TestCase):
    def test_returns_largest(self):
        self.assertEqual(
            max_by([{"n": 1}, {"n": 3}, {"n": 2}], lambda item: item["n"]), {"n": 3}
        )

    def test_returns_none_for_empty(self):
        self.assertIsNone(max_by([], lambda n: n))

    def test_single_element(self):
        self.assertEqual(max_by([7], lambda n: n), 7)

    def test_first_wins_ties(self):
        first = {"n": 2, "id": "first"}
        second = {"n": 2, "id": "second"}
        self.assertIs(
            max_by([{"n": 1, "id": "x"}, first, second], lambda item: item["n"]),
            first,
        )

    def test_compares_strings(self):
        self.assertEqual(max_by(["pear", "apple", "fig"], lambda s: s), "pear")

    def test_nan_never_replaces_maximum(self):
        self.assertEqual(max_by([1.0, math.nan, 2.0], lambda n: n), 2.0)
        result = max_by([math.nan, 1.0], lambda n: n)
        self.assertTrue(result is not None and math.isnan(result))


if __name__ == "__main__":
    unittest.main()
