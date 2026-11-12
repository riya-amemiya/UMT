import unittest

from src.string import dedent, indent


class TestIndent(unittest.TestCase):
    def test_docstring_examples(self):
        self.assertEqual(indent("line1\nline2", 2), "  line1\n  line2")
        self.assertEqual(indent("a\n\nb", 1, "\t"), "\ta\n\n\tb")

    def test_whitespace_only_line_is_prefixed(self):
        self.assertEqual(indent("  \nx", 2), "    \n  x")

    def test_multi_character_unit(self):
        self.assertEqual(indent("x", 2, "ab"), "ababx")

    def test_zero_or_negative_count(self):
        self.assertEqual(indent("a\nb", 0), "a\nb")
        self.assertEqual(indent("a\nb", -1), "a\nb")

    def test_empty_input(self):
        self.assertEqual(indent("", 4), "")
        self.assertEqual(indent("\n", 4), "\n")

    def test_reversed_by_dedent(self):
        original = "a\n  b\n\nc"
        self.assertEqual(dedent(indent(original, 4)), original)


if __name__ == "__main__":
    unittest.main()
