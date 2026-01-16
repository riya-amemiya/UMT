import unittest

from src.array import compact


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


if __name__ == "__main__":
    unittest.main()
