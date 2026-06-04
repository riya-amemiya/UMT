import unittest

from src.string import strip_ansi


class TestStripAnsi(unittest.TestCase):
    def test_removes_sgr_color_codes(self):
        self.assertEqual(strip_ansi("\x1b[31mred\x1b[0m"), "red")

    def test_removes_multiple_sequences(self):
        self.assertEqual(strip_ansi("\x1b[1m\x1b[32mbold green\x1b[0m"), "bold green")

    def test_removes_cursor_movement(self):
        self.assertEqual(strip_ansi("a\x1b[2Ab"), "ab")

    def test_leaves_plain_text(self):
        self.assertEqual(strip_ansi("plain text"), "plain text")

    def test_empty_string(self):
        self.assertEqual(strip_ansi(""), "")


if __name__ == "__main__":
    unittest.main()
