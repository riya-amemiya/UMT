import unittest

from src import delete_spaces


class TestDeleteSpaces(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(delete_spaces("Hello World"), "HelloWorld")
        self.assertEqual(
            delete_spaces("  leading and trailing spaces  "), "leadingandtrailingspaces"
        )
        self.assertEqual(delete_spaces("no_spaces"), "no_spaces")

    def test_edge_cases(self):
        self.assertEqual(delete_spaces(""), "")
        self.assertEqual(delete_spaces("   "), "")
        self.assertEqual(delete_spaces("\t\n\r\f\v"), "")  # All standard whitespace

    def test_docstring_example(self):
        self.assertEqual(delete_spaces("Hello World"), "HelloWorld")
        self.assertEqual(delete_spaces("  tab\t space "), "tabspace")


if __name__ == "__main__":
    unittest.main()
