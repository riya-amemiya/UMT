import unittest

from src.url import join_path


class TestJoinPathParity(unittest.TestCase):
    def test_slash_before_trailing_newline_is_kept(self):
        self.assertEqual(join_path("a/\n", "b"), "a/\n/b")
        self.assertEqual(join_path("a/", "\n/b/\n"), "a/\n/b/\n")

    def test_newline_before_leading_slash_is_kept(self):
        self.assertEqual(join_path("a", "/\nb"), "a/\nb")

    def test_last_segment_keeps_trailing_content(self):
        self.assertEqual(join_path("a", "b/\n"), "a/b/\n")

    def test_slash_only_and_empty_segments(self):
        self.assertEqual(join_path("/", "/"), "")
        self.assertEqual(join_path("", ""), "")
        self.assertEqual(join_path("a//", "//", "b//"), "a/b//")


if __name__ == "__main__":
    unittest.main()
