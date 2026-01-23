import unittest
import json

from src.tool import parse_json


class TestParseJson(unittest.TestCase):
    def test_basic_object(self):
        result = parse_json('{"a": 1}')
        self.assertEqual(result, {"a": 1})

    def test_array(self):
        result = parse_json("[1, 2, 3]")
        self.assertEqual(result, [1, 2, 3])

    def test_string(self):
        result = parse_json('"hello"')
        self.assertEqual(result, "hello")

    def test_number(self):
        result = parse_json("42")
        self.assertEqual(result, 42)

    def test_boolean(self):
        result = parse_json("true")
        self.assertEqual(result, True)
        result = parse_json("false")
        self.assertEqual(result, False)

    def test_null(self):
        result = parse_json("null")
        self.assertIsNone(result)

    def test_nested_object(self):
        result = parse_json('{"a": {"b": [1, 2, 3]}}')
        self.assertEqual(result, {"a": {"b": [1, 2, 3]}})

    def test_invalid_json(self):
        with self.assertRaises(json.JSONDecodeError):
            parse_json("invalid json")

    def test_docstring_example(self):
        self.assertEqual(parse_json('{"a": 1}'), {"a": 1})


if __name__ == "__main__":
    unittest.main()
