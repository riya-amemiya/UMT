import unittest

from src.url import join_path


class TestJoinPath(unittest.TestCase):
    def test_join_simple_segments(self):
        self.assertEqual(join_path("a", "b", "c"), "a/b/c")

    def test_normalize_slashes(self):
        self.assertEqual(join_path("a/", "/b/", "/c"), "a/b/c")

    def test_preserve_leading_slash(self):
        self.assertEqual(join_path("/a", "b", "c"), "/a/b/c")

    def test_preserve_trailing_slash(self):
        self.assertEqual(join_path("a", "b", "c/"), "a/b/c/")

    def test_handle_url_like_base(self):
        self.assertEqual(
            join_path("https://example.com/", "/api/", "/users"),
            "https://example.com/api/users",
        )

    def test_handle_single_segment(self):
        self.assertEqual(join_path("path"), "path")

    def test_return_empty_string_for_no_segments(self):
        self.assertEqual(join_path(), "")

    def test_handle_multiple_slashes(self):
        self.assertEqual(join_path("a///", "///b"), "a/b")

    def test_handle_empty_segments(self):
        self.assertEqual(join_path("a", "", "b"), "a/b")


if __name__ == "__main__":
    unittest.main()
