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

    def test_multiple_internal_spaces(self):
        self.assertEqual(delete_spaces("   Hello   World   "), "HelloWorld")

    def test_tabs_and_newlines(self):
        self.assertEqual(delete_spaces("Hello\tWorld"), "HelloWorld")
        self.assertEqual(delete_spaces("Hello\nWorld\r\n"), "HelloWorld")

    def test_unicode_whitespace(self):
        self.assertEqual(delete_spaces("Hello\u2003World"), "HelloWorld")
        self.assertEqual(delete_spaces("Hello\u3000World"), "HelloWorld")

    def test_multibyte_with_space(self):
        self.assertEqual(delete_spaces("こんにちは 世界"), "こんにちは世界")

    def test_mixed_consecutive_whitespace(self):
        self.assertEqual(delete_spaces("Hello\u2003\u3000 World\t\n"), "HelloWorld")


if __name__ == "__main__":
    unittest.main()
