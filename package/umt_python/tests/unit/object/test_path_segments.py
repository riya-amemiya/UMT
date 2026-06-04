import unittest

from src.object import path_segments


class TestPathSegments(unittest.TestCase):
    def test_splits_dotted_string_into_segments(self):
        self.assertEqual(path_segments("a.b.c"), ["a", "b", "c"])

    def test_returns_input_list_unchanged(self):
        source = ["a", "b"]
        result = path_segments(source)
        self.assertIs(result, source)

    def test_returns_single_element_for_non_dotted_string(self):
        self.assertEqual(path_segments("a"), ["a"])

    def test_splits_empty_string_into_single_empty_segment(self):
        self.assertEqual(path_segments(""), [""])

    def test_docstring_example(self):
        self.assertEqual(path_segments("a.b.c"), ["a", "b", "c"])
        self.assertEqual(path_segments(["a", "b"]), ["a", "b"])
        self.assertEqual(path_segments("a"), ["a"])
        self.assertEqual(path_segments(""), [""])


if __name__ == "__main__":
    unittest.main()
