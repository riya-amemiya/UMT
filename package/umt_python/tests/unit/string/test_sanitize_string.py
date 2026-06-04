import unittest

from src.string import sanitize_string


class TestSanitizeString(unittest.TestCase):
    def test_keeps_printable_ascii(self):
        self.assertEqual(sanitize_string("Hello, World!"), "Hello, World!")

    def test_keeps_full_printable_range(self):
        printable = (
            " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`"
            "abcdefghijklmnopqrstuvwxyz{|}~"
        )
        self.assertEqual(sanitize_string(printable), printable)

    def test_keeps_newline_cr_tab(self):
        self.assertEqual(
            sanitize_string("line1\nline2\r\n\tindented"),
            "line1\nline2\r\n\tindented",
        )

    def test_removes_ansi_escape_sequences(self):
        self.assertEqual(sanitize_string("\x1b[31mred\x1b[0m"), "[31mred[0m")

    def test_removes_null_bytes(self):
        self.assertEqual(sanitize_string("a\x00b"), "ab")

    def test_removes_c0_control_except_tab_lf_cr(self):
        self.assertEqual(sanitize_string("a\x01b\x02c\x1fd"), "abcd")

    def test_removes_del_character(self):
        self.assertEqual(sanitize_string("a\x7fb"), "ab")

    def test_removes_non_ascii_characters(self):
        self.assertEqual(sanitize_string("héllo"), "hllo")
        self.assertEqual(sanitize_string("日本語"), "")
        self.assertEqual(sanitize_string("emoji\U0001f600here"), "emojihere")

    def test_empty_string(self):
        self.assertEqual(sanitize_string(""), "")


if __name__ == "__main__":
    unittest.main()
