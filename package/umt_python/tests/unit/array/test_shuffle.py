import unittest

from src.array import shuffle


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


if __name__ == "__main__":
    unittest.main()
