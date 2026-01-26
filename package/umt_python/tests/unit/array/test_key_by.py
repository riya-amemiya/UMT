import unittest

from src.array import key_by


class MyObject:
    """Helper class for testing attribute access."""

    def __init__(self, name: str):
        self.name = name


class TestKeyBy(unittest.TestCase):
    def test_with_none_iteratee(self):
        """Test with None iteratee (uses str(value) as key)."""
        result = key_by([1, 2, 3])
        self.assertEqual(result, {"1": 1, "2": 2, "3": 3})

    def test_with_callable_iteratee(self):
        """Test with callable iteratee."""
        result = key_by([{"id": "a"}, {"id": "b"}], lambda x: x["id"])
        self.assertEqual(result, {"a": {"id": "a"}, "b": {"id": "b"}})

    def test_with_string_key_dict(self):
        """Test with string key on dict values."""
        result = key_by([{"id": "a"}, {"id": "b"}], "id")
        self.assertEqual(result, {"a": {"id": "a"}, "b": {"id": "b"}})

    def test_with_string_key_object(self):
        """Test with string key on object attributes."""
        obj1 = MyObject("alice")
        obj2 = MyObject("bob")
        result = key_by([obj1, obj2], "name")
        self.assertEqual(result["alice"], obj1)
        self.assertEqual(result["bob"], obj2)

    def test_with_dict_collection(self):
        """Test with dict collection instead of list."""
        collection = {"x": {"id": "a"}, "y": {"id": "b"}}
        result = key_by(collection, "id")
        self.assertEqual(result, {"a": {"id": "a"}, "b": {"id": "b"}})

    def test_dict_collection_with_callable(self):
        """Test with dict collection and callable iteratee."""
        collection = {"x": 1, "y": 2, "z": 3}
        result = key_by(collection, lambda x: str(x * 10))
        self.assertEqual(result, {"10": 1, "20": 2, "30": 3})

    def test_dict_collection_with_none_iteratee(self):
        """Test dict collection with None iteratee."""
        collection = {"x": 10, "y": 20}
        result = key_by(collection)
        self.assertEqual(result, {"10": 10, "20": 20})

    def test_empty_list(self):
        """Test with empty list."""
        result = key_by([])
        self.assertEqual(result, {})

    def test_empty_dict(self):
        """Test with empty dict."""
        result = key_by({})
        self.assertEqual(result, {})

    def test_duplicate_keys(self):
        """Test that later values override earlier ones with same key."""
        result = key_by([{"id": "a", "val": 1}, {"id": "a", "val": 2}], "id")
        self.assertEqual(result["a"]["val"], 2)

    def test_docstring_examples(self):
        """Test examples from docstring."""
        self.assertEqual(
            key_by([{"id": "a"}, {"id": "b"}], "id"),
            {"a": {"id": "a"}, "b": {"id": "b"}},
        )
        self.assertEqual(key_by([1, 2, 3]), {"1": 1, "2": 2, "3": 3})


if __name__ == "__main__":
    unittest.main()
