import unittest

from src.url import parse_query_string


class TestParseQueryStringParity(unittest.TestCase):
    def test_keeps_prototype_like_keys(self):
        self.assertEqual(
            parse_query_string("?__proto__=1&constructor=2&prototype=3&ok=4"),
            {"__proto__": "1", "constructor": "2", "prototype": "3", "ok": "4"},
        )

    def test_last_duplicate_wins(self):
        self.assertEqual(parse_query_string("a=1&a=2"), {"a": "2"})

    def test_pairs_without_key_or_value(self):
        self.assertEqual(parse_query_string("a"), {"a": ""})
        self.assertEqual(parse_query_string("=v"), {"": "v"})
        self.assertEqual(parse_query_string("a=b=c"), {"a": "b=c"})
        self.assertEqual(parse_query_string("&&a=1&&"), {"a": "1"})

    def test_percent_and_plus_decoding(self):
        self.assertEqual(
            parse_query_string("a=%E3%81%82"), {"a": "\N{HIRAGANA LETTER A}"}
        )
        self.assertEqual(parse_query_string("a+b=c+d"), {"a b": "c d"})
        self.assertEqual(parse_query_string("a=%2B"), {"a": "+"})
        self.assertEqual(parse_query_string("a=%4a%4A"), {"a": "JJ"})

    def test_malformed_percent_sequences_stay_literal(self):
        self.assertEqual(parse_query_string("a=%ZZ"), {"a": "%ZZ"})
        self.assertEqual(parse_query_string("a=%"), {"a": "%"})
        self.assertEqual(parse_query_string("a=%4"), {"a": "%4"})
        self.assertEqual(parse_query_string("a=%a\xe9"), {"a": "%a\xe9"})

    def test_invalid_utf8_becomes_replacement_character(self):
        self.assertEqual(
            parse_query_string("a=%ff"), {"a": "\N{REPLACEMENT CHARACTER}"}
        )

    def test_only_one_leading_question_mark_is_removed(self):
        self.assertEqual(parse_query_string("??a=1"), {"?a": "1"})

    def test_full_url_ignores_fragment(self):
        self.assertEqual(parse_query_string("https://e.com?a=1#frag"), {"a": "1"})
        self.assertEqual(parse_query_string("https://e.com/#/route?a=1"), {})

    def test_url_marker_without_scheme_raises(self):
        with self.assertRaises(ValueError):
            parse_query_string("redirect=https://x.com")
        with self.assertRaises(ValueError):
            parse_query_string("?next=http://x.com")


if __name__ == "__main__":
    unittest.main()
