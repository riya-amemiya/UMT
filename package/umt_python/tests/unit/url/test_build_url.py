import unittest

from src.url import build_url


class TestBuildUrl(unittest.TestCase):
    def test_build_url_with_query_parameters(self):
        result = build_url("https://example.com", {"page": "1", "q": "search"})
        self.assertEqual(result, "https://example.com/?page=1&q=search")

    def test_return_base_url_when_no_params(self):
        result = build_url("https://example.com/path")
        self.assertEqual(result, "https://example.com/path")

    def test_return_base_url_with_empty_params(self):
        result = build_url("https://example.com/path", {})
        self.assertEqual(result, "https://example.com/path")

    def test_encode_special_characters_in_values(self):
        result = build_url("https://example.com", {"q": "hello world"})
        self.assertIn("q=hello+world", result)

    def test_handle_existing_query_parameters_in_base(self):
        result = build_url("https://example.com?existing=1", {"added": "2"})
        self.assertIn("existing=1", result)
        self.assertIn("added=2", result)

    def test_handle_single_parameter(self):
        result = build_url("https://example.com", {"key": "value"})
        self.assertEqual(result, "https://example.com/?key=value")

    def test_skip_prototype_pollution_keys(self):
        result = build_url(
            "https://example.com",
            {
                "__proto__": "evil",
                "constructor": "evil",
                "prototype": "evil",
                "safe": "value",
            },
        )
        self.assertEqual(result, "https://example.com/?safe=value")
        self.assertNotIn("__proto__", result)
        self.assertNotIn("constructor", result)
        self.assertNotIn("prototype", result)


if __name__ == "__main__":
    unittest.main()
