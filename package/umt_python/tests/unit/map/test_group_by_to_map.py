import unittest

from src.map import group_by_to_map


class TestGroupByToMap(unittest.TestCase):
    def test_groups_numbers_into_odd_and_even(self):
        array = [1, 2, 3, 4, 5]
        result = group_by_to_map(
            array, lambda num, _i, _arr: "even" if num % 2 == 0 else "odd"
        )
        self.assertEqual(result, {"odd": [1, 3, 5], "even": [2, 4]})

    def test_preserves_insertion_order_of_keys(self):
        array = [3, 1, 2, 1, 3, 2]
        result = group_by_to_map(array, lambda num, _i, _arr: num)
        self.assertEqual(list(result.keys()), [3, 1, 2])

    def test_accepts_tuple_keys(self):
        a = (1, "a")
        b = (1, "b")
        array = [
            {"tag": a, "value": 1},
            {"tag": b, "value": 2},
            {"tag": a, "value": 3},
        ]
        result = group_by_to_map(array, lambda item, _i, _arr: item["tag"])
        self.assertEqual(
            result[a],
            [{"tag": a, "value": 1}, {"tag": a, "value": 3}],
        )
        self.assertEqual(result[b], [{"tag": b, "value": 2}])

    def test_groups_strings_by_length(self):
        array = ["one", "two", "three", "four", "five"]
        result = group_by_to_map(array, lambda s, _i, _arr: len(s))
        self.assertEqual(result[3], ["one", "two"])
        self.assertEqual(result[5], ["three"])
        self.assertEqual(result[4], ["four", "five"])

    def test_passes_index_and_array_to_iteratee(self):
        array = ["a", "b", "c", "d"]
        received_indices: list[int] = []

        def iteratee(_value: str, index: int, source: list[str]) -> int:
            self.assertIs(source, array)
            received_indices.append(index)
            return index % 2

        group_by_to_map(array, iteratee)
        self.assertEqual(received_indices, [0, 1, 2, 3])

    def test_returns_empty_dict_for_empty_array(self):
        empty: list[int] = []
        result = group_by_to_map(
            empty, lambda num, _i, _arr: "even" if num % 2 == 0 else "odd"
        )
        self.assertEqual(result, {})


if __name__ == "__main__":
    unittest.main()
