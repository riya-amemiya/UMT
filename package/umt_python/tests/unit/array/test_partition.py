import unittest

from src.array import partition


class TestPartition(unittest.TestCase):
    def test_splits_numbers_into_even_and_odd(self):
        result = partition([1, 2, 3, 4], lambda n, i, arr: n % 2 == 0)
        self.assertEqual(result, ([2, 4], [1, 3]))

    def test_puts_all_elements_in_pass_when_predicate_always_true(self):
        result = partition([1, 2, 3], lambda n, i, arr: True)
        self.assertEqual(result, ([1, 2, 3], []))

    def test_puts_all_elements_in_fail_when_predicate_always_false(self):
        result = partition([1, 2, 3], lambda n, i, arr: False)
        self.assertEqual(result, ([], [1, 2, 3]))

    def test_returns_empty_lists_for_empty_array(self):
        self.assertEqual(partition([], lambda n, i, arr: True), ([], []))

    def test_provides_index_and_array_to_the_predicate(self):
        source = ["a", "b", "c"]
        indices: list[int] = []
        arrays: list[list[str]] = []

        def predicate(_value: str, index: int, array: list[str]) -> bool:
            indices.append(index)
            arrays.append(array)
            return index % 2 == 0

        partition(source, predicate)
        self.assertEqual(indices, [0, 1, 2])
        self.assertTrue(all(array is source for array in arrays))

    def test_does_not_mutate_the_input_array(self):
        source = [1, 2, 3, 4]
        partition(source, lambda n, i, arr: n > 2)
        self.assertEqual(source, [1, 2, 3, 4])

    def test_preserves_object_references(self):
        a = {"id": 1}
        b = {"id": 2}
        c = {"id": 3}
        passed, failed = partition([a, b, c], lambda item, i, arr: item["id"] != 2)
        self.assertEqual(passed, [a, c])
        self.assertEqual(failed, [b])
        self.assertIs(passed[0], a)
        self.assertIs(failed[0], b)

    def test_handles_single_element_arrays(self):
        self.assertEqual(partition([1], lambda n, i, arr: n == 1), ([1], []))
        self.assertEqual(partition([1], lambda n, i, arr: n == 0), ([], [1]))

    def test_handles_mixed_types(self):
        passed, failed = partition(
            [0, "a", False, None, 1], lambda value, i, arr: bool(value)
        )
        self.assertEqual(passed, ["a", 1])
        self.assertEqual(failed, [0, False, None])

    def test_docstring_example(self):
        self.assertEqual(
            partition([1, 2, 3, 4], lambda n, i, arr: n % 2 == 0), ([2, 4], [1, 3])
        )


if __name__ == "__main__":
    unittest.main()
