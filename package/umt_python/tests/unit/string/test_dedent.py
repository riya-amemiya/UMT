import unittest

from src.string import dedent


class TestDedent(unittest.TestCase):
    def test_removes_minimum_common_indentation(self):
        self.assertEqual(
            dedent("    line1\n      line2\n    line3"),
            "line1\n  line2\nline3",
        )

    def test_ignores_blank_lines_when_computing_minimum(self):
        self.assertEqual(dedent("    a\n\n    b"), "a\n\nb")

    def test_unchanged_when_no_indentation(self):
        self.assertEqual(dedent("no indent"), "no indent")

    def test_all_blank_input(self):
        self.assertEqual(dedent("\n  \n"), "\n  \n")

    def test_tabs_count_as_indentation(self):
        self.assertEqual(dedent("\tline1\n\t\tline2"), "line1\n\tline2")


if __name__ == "__main__":
    unittest.main()
