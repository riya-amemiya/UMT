import unittest

from src.array import sliding


class TestSliding(unittest.TestCase):
    def test_returns_overlapping_windows_with_default_step(self):
        self.assertEqual(sliding([1, 2, 3, 4, 5], 3), [[1, 2, 3], [2, 3, 4], [3, 4, 5]])

    def test_respects_a_custom_step(self):
        self.assertEqual(sliding([1, 2, 3, 4, 5], 3, 2), [[1, 2, 3], [3, 4, 5]])

    def test_behaves_like_chunks_when_step_equals_size(self):
        self.assertEqual(sliding([1, 2, 3, 4, 5, 6], 2, 2), [[1, 2], [3, 4], [5, 6]])

    def test_omits_incomplete_trailing_windows(self):
        self.assertEqual(sliding([1, 2, 3, 4, 5], 3, 3), [[1, 2, 3]])

    def test_returns_empty_list_when_size_is_larger_than_length(self):
        self.assertEqual(sliding([1, 2], 5), [])

    def test_returns_empty_list_for_empty_input(self):
        self.assertEqual(sliding([], 2), [])

    def test_handles_size_of_one(self):
        self.assertEqual(sliding([1, 2, 3], 1), [[1], [2], [3]])

    def test_handles_size_equal_to_array_length(self):
        self.assertEqual(sliding([1, 2, 3], 3), [[1, 2, 3]])

    def test_handles_size_of_zero(self):
        self.assertEqual(sliding([1, 2], 0), [[], [], []])

    def test_does_not_mutate_the_input_array(self):
        source = [1, 2, 3, 4]
        sliding(source, 2)
        self.assertEqual(source, [1, 2, 3, 4])

    def test_preserves_object_references(self):
        a = {"id": 1}
        b = {"id": 2}
        c = {"id": 3}
        result = sliding([a, b, c], 2)
        self.assertEqual(result, [[a, b], [b, c]])
        self.assertIs(result[0][0], a)
        self.assertIs(result[1][1], c)

    def test_handles_string_arrays(self):
        self.assertEqual(
            sliding(["a", "b", "c", "d"], 2, 1),
            [["a", "b"], ["b", "c"], ["c", "d"]],
        )

    def test_handles_single_element_with_size_one(self):
        self.assertEqual(sliding([42], 1), [[42]])

    def test_docstring_example(self):
        self.assertEqual(sliding([1, 2, 3, 4, 5], 3), [[1, 2, 3], [2, 3, 4], [3, 4, 5]])
        self.assertEqual(sliding([1, 2, 3, 4, 5], 3, 2), [[1, 2, 3], [3, 4, 5]])


if __name__ == "__main__":
    unittest.main()
