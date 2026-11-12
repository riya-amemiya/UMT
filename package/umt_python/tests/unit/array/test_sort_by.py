import unittest

from src.array import sort_by


class TestSortBy(unittest.TestCase):
    def test_sorts_by_single_selector(self):
        self.assertEqual(
            sort_by([{"age": 30}, {"age": 20}, {"age": 25}], lambda u: u["age"]),
            [{"age": 20}, {"age": 25}, {"age": 30}],
        )

    def test_later_selectors_break_ties(self):
        self.assertEqual(
            sort_by(["bb", "a", "ab", "c"], len, lambda s: s),
            ["a", "c", "ab", "bb"],
        )

    def test_is_stable_for_equal_keys(self):
        items = [(1, "a"), (0, "b"), (1, "c"), (0, "d")]
        result = sort_by(items, lambda item: item[0])
        self.assertEqual([item[1] for item in result], ["b", "d", "a", "c"])

    def test_is_stable_for_large_inputs(self):
        items = [(index % 3, index) for index in range(200)]
        result = sort_by(items, lambda item: item[0])
        self.assertEqual(result, sorted(items, key=lambda item: item[0]))

    def test_returns_unchanged_copy_without_selectors(self):
        source = [3, 1, 2]
        result = sort_by(source)
        self.assertEqual(result, [3, 1, 2])
        self.assertIsNot(result, source)

    def test_empty_and_single_element(self):
        self.assertEqual(sort_by([], lambda n: n), [])
        self.assertEqual(sort_by([1], lambda n: n), [1])

    def test_calls_each_selector_once_per_element(self):
        calls: list[int] = []

        def selector(n: int) -> int:
            calls.append(n)
            return n

        sort_by([5, 3, 4, 1, 2], selector)
        self.assertEqual(len(calls), 5)

    def test_does_not_mutate_input(self):
        source = [3, 1, 2]
        sort_by(source, lambda n: n)
        self.assertEqual(source, [3, 1, 2])

    def test_descending_through_negated_key(self):
        self.assertEqual(sort_by([1, 3, 2], lambda n: -n), [3, 2, 1])


if __name__ == "__main__":
    unittest.main()
