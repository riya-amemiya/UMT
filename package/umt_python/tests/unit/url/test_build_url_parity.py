import unittest

from src.url import build_url


class TestBuildUrlParity(unittest.TestCase):
    def test_encodes_like_url_search_params(self):
        self.assertEqual(
            build_url("https://example.com", {"a": "~*-._ !'()"}),
            "https://example.com/?a=%7E*-._+%21%27%28%29",
        )
        self.assertEqual(
            build_url("https://example.com", {"b": "\N{HIRAGANA LETTER A}/?&=#+%"}),
            "https://example.com/?b=%E3%81%82%2F%3F%26%3D%23%2B%25",
        )

    def test_keeps_prototype_like_keys(self):
        self.assertEqual(
            build_url("https://example.com", {"constructor": "c"}),
            "https://example.com/?constructor=c",
        )
        self.assertEqual(
            build_url("https://example.com", {"prototype": "p"}),
            "https://example.com/?prototype=p",
        )
        self.assertEqual(
            build_url("https://example.com", {"__proto__": "x"}),
            "https://example.com/?__proto__=x",
        )

    def test_query_goes_before_fragment(self):
        self.assertEqual(
            build_url("https://example.com/p#frag", {"k": "v"}),
            "https://example.com/p?k=v#frag",
        )
        self.assertEqual(
            build_url("https://example.com/p?x=1#frag", {"k": "v"}),
            "https://example.com/p?x=1&k=v#frag",
        )
        self.assertEqual(
            build_url("https://example.com#a?b", {"k": "v"}),
            "https://example.com/?k=v#a?b",
        )

    def test_existing_query_without_new_params(self):
        self.assertEqual(
            build_url("https://example.com/p?x=1"), "https://example.com/p?x=1"
        )
        self.assertEqual(build_url("https://example.com#f"), "https://example.com/#f")

    def test_root_path_is_added_only_for_special_schemes(self):
        self.assertEqual(
            build_url("ws://example.com", {"k": "v"}), "ws://example.com/?k=v"
        )
        self.assertEqual(build_url("foo://host", {"k": "v"}), "foo://host?k=v")
        self.assertEqual(
            build_url("mailto:user@host", {"k": "v"}), "mailto:user@host?k=v"
        )
        self.assertEqual(build_url("foo:", {"k": "v"}), "foo:?k=v")

    def test_empty_key_and_value(self):
        self.assertEqual(
            build_url("https://example.com", {"": ""}), "https://example.com/?="
        )

    def test_existing_query_is_serialized_again_when_params_are_appended(self):
        self.assertEqual(
            build_url("https://example.com/?a=1%20b", {"k": "v"}),
            "https://example.com/?a=1+b&k=v",
        )
        self.assertEqual(
            build_url("https://example.com/p?x=%7e#frag", {"k": "v"}),
            "https://example.com/p?x=%7E&k=v#frag",
        )
        self.assertEqual(
            build_url("https://e.com/?a=1&&b", {"k": "v"}),
            "https://e.com/?a=1&b=&k=v",
        )
        self.assertEqual(
            build_url("https://e.com/?a=%zz&b=%e9", {"k": "v"}),
            "https://e.com/?a=%25zz&b=%EF%BF%BD&k=v",
        )

    def test_existing_query_is_kept_verbatim_with_empty_params(self):
        self.assertEqual(
            build_url("https://example.com/?a=1%20b", {}),
            "https://example.com/?a=1%20b",
        )
        self.assertEqual(
            build_url("https://example.com/?a=1%20b"),
            "https://example.com/?a=1%20b",
        )

    def test_base_without_scheme_raises(self):
        with self.assertRaises(ValueError):
            build_url("/relative", {"k": "v"})
        with self.assertRaises(ValueError):
            build_url("")


if __name__ == "__main__":
    unittest.main()
