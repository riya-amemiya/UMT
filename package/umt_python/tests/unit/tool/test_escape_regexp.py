import unittest

from src.tool.escape_regexp import escape_regexp


class TestEscapeRegexp(unittest.TestCase):
    def test_escape_all_regex_special_characters(self):
        special_chars = ".*+?^${}()|[]\\"
        expected = r"\.\*\+\?\^\$\{\}\(\)\|\[\]\\"
        self.assertEqual(escape_regexp(special_chars), expected)

    def test_escape_each_character_correctly_individually(self):
        self.assertEqual(escape_regexp("."), "\\.")
        self.assertEqual(escape_regexp("*"), "\\*")
        self.assertEqual(escape_regexp("+"), "\\+")
        self.assertEqual(escape_regexp("?"), "\\?")
        self.assertEqual(escape_regexp("^"), "\\^")
        self.assertEqual(escape_regexp("$"), "\\$")
        self.assertEqual(escape_regexp("{"), "\\{")
        self.assertEqual(escape_regexp("}"), "\\}")
        self.assertEqual(escape_regexp("("), "\\(")
        self.assertEqual(escape_regexp(")"), "\\)")
        self.assertEqual(escape_regexp("|"), "\\|")
        self.assertEqual(escape_regexp("["), "\\[")
        self.assertEqual(escape_regexp("]"), "\\]")
        self.assertEqual(escape_regexp("\\"), "\\\\")

    def test_not_escape_alphanumeric_characters(self):
        string_ = "abcABC123"
        self.assertEqual(escape_regexp(string_), string_)

    def test_handle_mixed_strings(self):
        string_ = "a.b+c"
        self.assertEqual(escape_regexp(string_), "a\\.b\\+c")

    def test_handle_empty_string(self):
        self.assertEqual(escape_regexp(""), "")

    def test_docstring_example(self):
        self.assertEqual(escape_regexp("a.b"), "a\\.b")

    def test_consecutive_specials(self):
        self.assertEqual(escape_regexp("..."), "\\.\\.\\.")
        self.assertEqual(escape_regexp("???"), "\\?\\?\\?")

    def test_surrounding_specials(self):
        self.assertEqual(escape_regexp(".hello."), "\\.hello\\.")
        self.assertEqual(escape_regexp("^start$"), "\\^start\\$")

    def test_character_class_and_alternation(self):
        self.assertEqual(escape_regexp("foo[bar]"), "foo\\[bar\\]")
        self.assertEqual(escape_regexp("a|b|c"), "a\\|b\\|c")
        self.assertEqual(escape_regexp("x{1,2}"), "x\\{1,2\\}")

    def test_currency_and_groups(self):
        self.assertEqual(escape_regexp("$100"), "\\$100")
        self.assertEqual(escape_regexp("(group)|alt"), "\\(group\\)\\|alt")

    def test_hyphen_underscore_space_unchanged(self):
        self.assertEqual(escape_regexp("foo-bar_baz"), "foo-bar_baz")
        self.assertEqual(escape_regexp("hello world"), "hello world")
        self.assertEqual(escape_regexp("path/to/file.txt"), "path/to/file\\.txt")

    def test_multibyte_with_dot(self):
        self.assertEqual(escape_regexp("こんにちは.世界"), "こんにちは\\.世界")

    def test_already_escaped_backslash(self):
        self.assertEqual(escape_regexp(r"a\.b"), "a\\\\\\.b")


if __name__ == "__main__":
    unittest.main()
