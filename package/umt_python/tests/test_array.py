import unittest

from src.array import (
    array_sum,
    arrays_join,
    binary_search,
    chunk,
    compact,
    drop,
    first,
    generate_number_array,
    get_arrays_diff,
    group_by,
    pop,
    shuffle,
    uniq_by,
    unique,
    zip_arrays,
)


class TestChunk(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(
            chunk([1, 2, 3, 4, 5, 6, 7, 8, 9], 3), [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        )
        self.assertEqual(chunk([1, 2, 3, 4, 5], 2), [[1, 2], [3, 4], [5]])

    def test_edge_cases(self):
        self.assertEqual(chunk([], 3), [])
        self.assertEqual(chunk([1, 2, 3], 5), [[1, 2, 3]])
        self.assertEqual(chunk([1, 2, 3], 1), [[1], [2], [3]])

    def test_docstring_example(self):
        self.assertEqual(
            chunk([1, 2, 3, 4, 5, 6, 7, 8, 9], 3), [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        )
        self.assertEqual(chunk([1, 2, 3, 4, 5], 2), [[1, 2], [3, 4], [5]])

    def test_does_not_mutate_input(self):
        original = [1, 2, 3, 4, 5]
        chunk(original, 2)
        self.assertEqual(original, [1, 2, 3, 4, 5])


class TestBinarySearch(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(binary_search([1, 2, 3, 4, 5], 3), 2)
        self.assertEqual(binary_search([1, 2, 3, 4, 5], 1), 0)
        self.assertEqual(binary_search([1, 2, 3, 4, 5], 5), 4)

    def test_not_found(self):
        self.assertEqual(binary_search([1, 2, 3, 4, 5], 6), -1)
        self.assertEqual(binary_search([1, 2, 3, 4, 5], 0), -1)

    def test_edge_cases(self):
        self.assertEqual(binary_search([], 1), -1)
        self.assertEqual(binary_search([5], 5), 0)
        self.assertEqual(binary_search([5], 3), -1)

    def test_docstring_example(self):
        self.assertEqual(binary_search([1, 2, 3, 4, 5], 3), 2)
        self.assertEqual(binary_search([1, 2, 3, 4, 5], 6), -1)


class TestUnique(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(unique([1, 2, 2, 3, 3, 3]), [1, 2, 3])
        self.assertEqual(unique(["a", "b", "a", "c"]), ["a", "b", "c"])

    def test_edge_cases(self):
        self.assertEqual(unique([]), [])
        self.assertEqual(unique([1, 1, 1, 1]), [1])
        self.assertEqual(unique([1]), [1])

    def test_preserves_order(self):
        self.assertEqual(unique([3, 1, 2, 1, 3, 2]), [3, 1, 2])

    def test_docstring_example(self):
        self.assertEqual(unique([1, 2, 2, 3, 3, 3]), [1, 2, 3])
        self.assertEqual(unique(["a", "b", "a", "c"]), ["a", "b", "c"])


class TestShuffle(unittest.TestCase):
    def test_basic_cases(self):
        arr = [1, 2, 3, 4, 5]
        result = shuffle(arr)
        self.assertEqual(len(result), len(arr))
        self.assertEqual(set(result), set(arr))

    def test_does_not_mutate_input(self):
        original = [1, 2, 3, 4, 5]
        shuffle(original)
        self.assertEqual(original, [1, 2, 3, 4, 5])

    def test_edge_cases(self):
        self.assertEqual(shuffle([]), [])
        self.assertEqual(shuffle([1]), [1])

    def test_docstring_example(self):
        arr = [1, 2, 3, 4, 5]
        result = shuffle(arr)
        self.assertEqual(len(result), len(arr))
        self.assertEqual(set(result), set(arr))


class TestGroupBy(unittest.TestCase):
    def test_basic_cases(self):
        result = group_by([6.1, 4.2, 6.3], lambda x, i, arr: int(x))
        self.assertEqual(result[6], [6.1, 6.3])
        self.assertEqual(result[4], [4.2])

    def test_by_length(self):
        result = group_by(["one", "two", "three"], lambda x, i, arr: len(x))
        self.assertEqual(result[3], ["one", "two"])
        self.assertEqual(result[5], ["three"])

    def test_by_first_char(self):
        result = group_by(["apple", "banana", "carrot"], lambda x, i, arr: x[0])
        self.assertEqual(result["a"], ["apple"])
        self.assertEqual(result["b"], ["banana"])
        self.assertEqual(result["c"], ["carrot"])

    def test_edge_cases(self):
        self.assertEqual(group_by([], lambda x, i, arr: 0), {})

    def test_docstring_example(self):
        result = group_by([6.1, 4.2, 6.3], lambda x, i, arr: int(x))
        self.assertEqual(result, {6: [6.1, 6.3], 4: [4.2]})


class TestFirst(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(first([1, 2, 3]), 1)
        self.assertEqual(first(["a", "b", "c"]), "a")

    def test_edge_cases(self):
        self.assertIsNone(first([]))

    def test_docstring_example(self):
        self.assertEqual(first([1, 2, 3]), 1)
        self.assertIsNone(first([]))
        self.assertEqual(first(["a", "b", "c"]), "a")


class TestDrop(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(drop([1, 2, 3, 4, 5], 2), [3, 4, 5])
        self.assertEqual(drop([1, 2, 3, 4, 5], 2, "left"), [3, 4, 5])
        self.assertEqual(drop([1, 2, 3, 4, 5], 2, "right"), [1, 2, 3])

    def test_between_direction(self):
        self.assertEqual(drop([1, 2, 3, 4, 5], 1, "between"), [1, 2, 4, 5])

    def test_edge_cases(self):
        self.assertEqual(drop([1, 2, 3], -1), [1, 2, 3])
        self.assertEqual(drop([1, 2, 3], 5), [])

    def test_docstring_example(self):
        self.assertEqual(drop([1, 2, 3, 4, 5], 2), [3, 4, 5])
        self.assertEqual(drop([1, 2, 3, 4, 5], 2, "left"), [3, 4, 5])
        self.assertEqual(drop([1, 2, 3, 4, 5], 2, "right"), [1, 2, 3])
        self.assertEqual(drop([1, 2, 3, 4, 5], 1, "between"), [1, 2, 4, 5])


class TestZipArrays(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(zip_arrays([1, 2], ["a", "b"]), [[1, "a"], [2, "b"]])
        self.assertEqual(zip_arrays([1, 2, 3], ["a", "b"]), [[1, "a"], [2, "b"]])

    def test_edge_cases(self):
        self.assertEqual(zip_arrays(), [])
        self.assertEqual(zip_arrays([1, 2, 3]), [[1], [2], [3]])

    def test_docstring_example(self):
        self.assertEqual(zip_arrays([1, 2], ["a", "b"]), [[1, "a"], [2, "b"]])
        self.assertEqual(zip_arrays([1, 2, 3], ["a", "b"]), [[1, "a"], [2, "b"]])


class TestCompact(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(compact([0, 1, False, 2, "", 3]), [1, 2, 3])
        self.assertEqual(compact([None, 0, False, ""]), [])

    def test_edge_cases(self):
        self.assertEqual(compact([]), [])
        self.assertEqual(compact([1, 2, 3]), [1, 2, 3])

    def test_docstring_example(self):
        self.assertEqual(compact([0, 1, False, 2, "", 3]), [1, 2, 3])
        self.assertEqual(compact([None, 0, False, ""]), [])


class TestPop(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(pop([1, 2, 3]), 3)
        self.assertEqual(pop(["a", "b"]), "b")

    def test_edge_cases(self):
        self.assertIsNone(pop([]))

    def test_docstring_example(self):
        self.assertEqual(pop([1, 2, 3]), 3)
        self.assertIsNone(pop([]))
        self.assertEqual(pop(["a", "b"]), "b")


class TestArraysJoin(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(arrays_join([1, 2, 3], [2, 3, 4]), [1, 2, 3, 4])
        self.assertEqual(arrays_join([1, 2], [2, 3], [3, 4]), [1, 2, 3, 4])

    def test_edge_cases(self):
        self.assertEqual(arrays_join([], []), [])
        self.assertEqual(arrays_join([1, 2, 3]), [1, 2, 3])

    def test_docstring_example(self):
        self.assertEqual(arrays_join([1, 2, 3], [2, 3, 4]), [1, 2, 3, 4])
        self.assertEqual(arrays_join([1, 2], [2, 3], [3, 4]), [1, 2, 3, 4])


class TestArraySum(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(array_sum([1, 2, 3]), 6)
        self.assertEqual(array_sum([1.5, 2.5, 3.0]), 7.0)

    def test_edge_cases(self):
        self.assertEqual(array_sum([]), 0)
        self.assertEqual(array_sum([5]), 5)

    def test_docstring_example(self):
        self.assertEqual(array_sum([1, 2, 3]), 6)
        self.assertEqual(array_sum([1.5, 2.5, 3.0]), 7.0)


class TestGetArraysDiff(unittest.TestCase):
    def test_basic_cases(self):
        result = get_arrays_diff([1, 2, 3], [2, 3, 4])
        self.assertEqual(sorted(result), [1, 4])

    def test_edge_cases(self):
        self.assertEqual(get_arrays_diff([1, 2, 3], [1, 2, 3]), [])
        result = get_arrays_diff([1, 2], [3, 4])
        self.assertEqual(sorted(result), [1, 2, 3, 4])

    def test_docstring_example(self):
        result = get_arrays_diff([1, 2, 3], [2, 3, 4])
        self.assertEqual(sorted(result), [1, 4])


class TestGenerateNumberArray(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(generate_number_array(5), [0, 1, 2, 3, 4])
        self.assertEqual(generate_number_array(5, 10, 14), [10, 11, 12, 13, 14])

    def test_edge_cases(self):
        self.assertEqual(generate_number_array(0), [])
        self.assertEqual(generate_number_array(1), [0])

    def test_invalid_input(self):
        with self.assertRaises(ValueError):
            generate_number_array(5, 10, 5)

    def test_random_values(self):
        result = generate_number_array(5, 0, 10, random_values=True)
        self.assertEqual(len(result), 5)
        for val in result:
            self.assertTrue(0 <= val <= 10)

    def test_docstring_example(self):
        self.assertEqual(generate_number_array(5), [0, 1, 2, 3, 4])
        self.assertEqual(generate_number_array(5, 10, 14), [10, 11, 12, 13, 14])


class TestUniqBy(unittest.TestCase):
    def test_basic_cases(self):
        result = uniq_by([{"x": 1}, {"x": 2}, {"x": 1}], lambda item: item["x"])
        self.assertEqual(result, [{"x": 1}, {"x": 2}])

    def test_edge_cases(self):
        self.assertEqual(uniq_by([], lambda x: x), [])
        result = uniq_by([1, 1, 1], lambda x: x)
        self.assertEqual(result, [1])

    def test_docstring_example(self):
        result = uniq_by([{"x": 1}, {"x": 2}, {"x": 1}], lambda item: item["x"])
        self.assertEqual(result, [{"x": 1}, {"x": 2}])


if __name__ == "__main__":
    unittest.main()
