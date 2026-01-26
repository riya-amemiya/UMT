import unittest

from src.array import group_by


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


if __name__ == "__main__":
    unittest.main()
