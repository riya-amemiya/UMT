import unittest

from src.url import parse_query_string


class TestParseQueryString(unittest.TestCase):
    def test_parse_with_leading_question_mark(self):
        result = parse_query_string("?page=1&q=search")
        self.assertEqual(result, {"page": "1", "q": "search"})

    def test_parse_without_leading_question_mark(self):
        result = parse_query_string("foo=bar&baz=qux")
        self.assertEqual(result, {"foo": "bar", "baz": "qux"})

    def test_parse_full_url(self):
        result = parse_query_string("https://example.com?a=1&b=2")
        self.assertEqual(result, {"a": "1", "b": "2"})

    def test_empty_string(self):
        result = parse_query_string("")
        self.assertEqual(result, {})

    def test_question_mark_only(self):
        result = parse_query_string("?")
        self.assertEqual(result, {})

    def test_encoded_values(self):
        result = parse_query_string("?q=hello%20world")
        self.assertEqual(result, {"q": "hello world"})

    def test_single_parameter(self):
        result = parse_query_string("?key=value")
        self.assertEqual(result, {"key": "value"})

    def test_url_with_no_query_string(self):
        result = parse_query_string("https://example.com/path")
        self.assertEqual(result, {})

    def test_reject_proto_key(self):
        result = parse_query_string("?__proto__=polluted&safe=value")
        self.assertEqual(result, {"safe": "value"})
        self.assertNotIn("__proto__", result)

    def test_reject_constructor_and_prototype_keys(self):
        result = parse_query_string("?constructor=bad&prototype=bad&ok=good")
        self.assertEqual(result, {"ok": "good"})
        self.assertNotIn("constructor", result)
        self.assertNotIn("prototype", result)


if __name__ == "__main__":
    unittest.main()
