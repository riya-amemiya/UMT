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


if __name__ == "__main__":
    unittest.main()
