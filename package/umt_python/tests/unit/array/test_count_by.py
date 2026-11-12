import math
import unittest

from src.array import count_by


class TestCountBy(unittest.TestCase):
    def test_counts_numbers_by_floor(self):
        result = count_by([6.1, 4.2, 6.3], lambda x, i, arr: math.floor(x))
        self.assertEqual(result, {4: 1, 6: 2})

    def test_counts_strings_by_length(self):
        result = count_by(["one", "two", "three"], lambda x, i, arr: len(x))
        self.assertEqual(result, {3: 2, 5: 1})

    def test_counts_objects_by_property(self):
        array = [{"type": "fruit"}, {"type": "vegetable"}, {"type": "fruit"}]
        result = count_by(array, lambda item, i, arr: item["type"])
        self.assertEqual(result, {"fruit": 2, "vegetable": 1})

    def test_returns_empty_dict_for_empty_array(self):
        self.assertEqual(count_by([], lambda x, i, arr: 0), {})

    def test_counts_all_under_one_key_when_iteratee_is_constant(self):
        self.assertEqual(count_by([1, 2, 3], lambda x, i, arr: "all"), {"all": 3})

    def test_provides_index_and_array_to_the_iteratee(self):
        source = ["a", "b", "c"]
        indices: list[int] = []
        arrays: list[list[str]] = []

        def iteratee(_value: str, index: int, array: list[str]) -> int:
            indices.append(index)
            arrays.append(array)
            return index

        count_by(source, iteratee)
        self.assertEqual(indices, [0, 1, 2])
        self.assertTrue(all(array is source for array in arrays))

    def test_handles_numeric_keys(self):
        result = count_by([1, 2, 1, 3, 2, 1], lambda x, i, arr: x)
        self.assertEqual(result, {1: 3, 2: 2, 3: 1})

    def test_handles_boolean_derived_string_keys(self):
        result = count_by(
            [True, False, True], lambda value, i, arr: "yes" if value else "no"
        )
        self.assertEqual(result, {"yes": 2, "no": 1})

    def test_handles_single_element_array(self):
        self.assertEqual(count_by(["x"], lambda x, i, arr: x), {"x": 1})

    def test_preserves_first_seen_key_order(self):
        result = count_by(["b", "a", "b"], lambda x, i, arr: x)
        self.assertEqual(list(result), ["b", "a"])

    def test_docstring_example(self):
        self.assertEqual(
            count_by([6.1, 4.2, 6.3], lambda x, i, arr: int(x)), {6: 2, 4: 1}
        )
        self.assertEqual(
            count_by(["one", "two", "three"], lambda x, i, arr: len(x)), {3: 2, 5: 1}
        )


if __name__ == "__main__":
    unittest.main()
