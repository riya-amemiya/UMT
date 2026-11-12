import unittest

from src.url import build_url, parse_query_string, stringify_query


class TestStringifyQuery(unittest.TestCase):
    def test_serializes_pairs_in_insertion_order(self):
        self.assertEqual(
            stringify_query({"page": "1", "q": "search"}), "page=1&q=search"
        )

    def test_empty_dictionary(self):
        self.assertEqual(stringify_query({}), "")

    def test_form_urlencoded(self):
        self.assertEqual(
            stringify_query(
                {
                    "q": "hello world",
                    "t": "a&b=c",
                    "s": "~*-._!'()",
                    "u": "\N{HIRAGANA LETTER A}",
                    "e": "",
                }
            ),
            "q=hello+world&t=a%26b%3Dc&s=%7E*-._%21%27%28%29&u=%E3%81%82&e=",
        )

    def test_encodes_keys(self):
        self.assertEqual(stringify_query({"a b": "1", "c&d": "2"}), "a+b=1&c%26d=2")

    def test_matches_build_url(self):
        params = {"q": "hello world", "tag": "a&b", "emoji": "\U0001f600"}
        self.assertEqual(
            f"https://example.com/?{stringify_query(params)}",
            build_url("https://example.com", params),
        )

    def test_inverse_of_parse_query_string(self):
        params = {"q": "hello world", "tag": "a&b=c", "plus": "1+1"}
        self.assertEqual(parse_query_string(stringify_query(params)), params)


if __name__ == "__main__":
    unittest.main()
