import unittest

from src.array import uniq_by


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
