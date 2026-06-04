import unittest

from src.string import get_value


class TestGetValueSimple(unittest.TestCase):
    def test_simple_properties(self):
        obj = {"name": "Alice", "age": 30}
        self.assertEqual(get_value(obj, "name"), "Alice")
        self.assertEqual(get_value(obj, "age"), 30)

    def test_non_existent_properties(self):
        obj = {"name": "Alice"}
        self.assertIsNone(get_value(obj, "nonexistent"))
        self.assertIsNone(get_value(obj, "missing"))

    def test_various_data_types(self):
        obj = {
            "string": "text",
            "number": 42,
            "boolean": True,
            "nullValue": None,
            "zero": 0,
            "emptyString": "",
        }
        self.assertEqual(get_value(obj, "string"), "text")
        self.assertEqual(get_value(obj, "number"), 42)
        self.assertEqual(get_value(obj, "boolean"), True)
        self.assertIsNone(get_value(obj, "nullValue"))
        self.assertEqual(get_value(obj, "zero"), 0)
        self.assertEqual(get_value(obj, "emptyString"), "")


class TestGetValueNested(unittest.TestCase):
    def test_nested_properties(self):
        obj = {"user": {"name": "Bob", "profile": {"age": 25, "location": "Tokyo"}}}
        self.assertEqual(get_value(obj, "user.name"), "Bob")
        self.assertEqual(get_value(obj, "user.profile.age"), 25)
        self.assertEqual(get_value(obj, "user.profile.location"), "Tokyo")

    def test_broken_nested_paths(self):
        obj = {"user": {"name": "Bob"}}
        self.assertIsNone(get_value(obj, "user.nonexistent"))
        self.assertIsNone(get_value(obj, "user.profile.age"))
        self.assertIsNone(get_value(obj, "nonexistent.property"))

    def test_deep_nesting(self):
        obj = {"level1": {"level2": {"level3": {"level4": {"value": "deep"}}}}}
        self.assertEqual(get_value(obj, "level1.level2.level3.level4.value"), "deep")


class TestGetValueArray(unittest.TestCase):
    def test_positive_indices(self):
        obj = {"items": ["A", "B", "C", "D"]}
        self.assertEqual(get_value(obj, "items[0]"), "A")
        self.assertEqual(get_value(obj, "items[1]"), "B")
        self.assertEqual(get_value(obj, "items[2]"), "C")
        self.assertEqual(get_value(obj, "items[3]"), "D")

    def test_negative_indices(self):
        obj = {"items": ["A", "B", "C", "D"]}
        self.assertEqual(get_value(obj, "items[-1]"), "D")
        self.assertEqual(get_value(obj, "items[-2]"), "C")
        self.assertEqual(get_value(obj, "items[-3]"), "B")
        self.assertEqual(get_value(obj, "items[-4]"), "A")

    def test_out_of_bounds(self):
        obj = {"items": ["A", "B"]}
        self.assertIsNone(get_value(obj, "items[5]"))
        self.assertIsNone(get_value(obj, "items[-5]"))

    def test_empty_arrays(self):
        obj = {"items": []}
        self.assertIsNone(get_value(obj, "items[0]"))
        self.assertIsNone(get_value(obj, "items[-1]"))


class TestGetValueComplex(unittest.TestCase):
    def test_arrays_of_objects(self):
        obj = {
            "users": [
                {"name": "Alice", "age": 30},
                {"name": "Bob", "age": 25},
                {"name": "Charlie", "age": 35},
            ]
        }
        self.assertEqual(get_value(obj, "users[0].name"), "Alice")
        self.assertEqual(get_value(obj, "users[1].age"), 25)
        self.assertEqual(get_value(obj, "users[-1].name"), "Charlie")

    def test_nested_arrays(self):
        obj = {"matrix": [[1, 2, 3], [4, 5, 6], [7, 8, 9]]}
        self.assertEqual(get_value(obj, "matrix[0]"), [1, 2, 3])
        self.assertEqual(get_value(obj, "matrix[1]"), [4, 5, 6])
        self.assertEqual(get_value(obj, "matrix[-1]"), [7, 8, 9])

    def test_objects_containing_arrays_containing_objects(self):
        obj = {
            "data": {
                "categories": [
                    {
                        "name": "Technology",
                        "items": [
                            {"title": "Laptop", "price": 1000},
                            {"title": "Phone", "price": 500},
                        ],
                    },
                    {"name": "Books", "items": [{"title": "Novel", "price": 20}]},
                ]
            }
        }
        self.assertEqual(get_value(obj, "data.categories[0].name"), "Technology")
        self.assertEqual(get_value(obj, "data.categories[0].items[1].title"), "Phone")
        self.assertEqual(get_value(obj, "data.categories[-1].items[0].price"), 20)


class TestGetValueEdgeCases(unittest.TestCase):
    def test_null_and_undefined_objects(self):
        self.assertIsNone(get_value(None, "property"))

    def test_primitive_values_as_objects(self):
        self.assertIsNone(get_value(42, "property"))
        self.assertIsNone(get_value("string", "property"))
        self.assertIsNone(get_value(True, "property"))

    def test_empty_path(self):
        obj = {"name": "Alice"}
        self.assertIsNone(get_value(obj, ""))

    def test_array_access_on_non_arrays(self):
        obj = {"notAnArray": "string"}
        self.assertIsNone(get_value(obj, "notAnArray[0]"))

    def test_complex_array_notation(self):
        obj = {"complex[key]": "value", "items": ["A", "B"]}
        self.assertEqual(get_value(obj, "items[0]"), "A")
        self.assertEqual(get_value(obj, "complex[key]"), "value")


class TestGetValueSpecialPaths(unittest.TestCase):
    def test_zero_indices(self):
        obj = {"items": ["zero", "one", "two"]}
        self.assertEqual(get_value(obj, "items[0]"), "zero")
        self.assertEqual(get_value(obj, "items[-0]"), "zero")

    def test_array_indices_in_middle_of_paths(self):
        obj = {
            "groups": [
                {
                    "name": "Group1",
                    "subGroups": [
                        {"name": "SubGroup1", "value": "test1"},
                        {"name": "SubGroup2", "value": "test2"},
                    ],
                }
            ]
        }
        self.assertEqual(get_value(obj, "groups[0].subGroups[1].value"), "test2")


class TestGetValuePrototypePollution(unittest.TestCase):
    def test_block_proto_key_access(self):
        obj = {"a": {"b": 1}}
        self.assertIsNone(get_value(obj, "__proto__"))
        self.assertIsNone(get_value(obj, "__proto__.polluted"))
        self.assertIsNone(get_value(obj, "a.__proto__"))

    def test_block_constructor_key_access(self):
        obj = {"a": {"b": 1}}
        self.assertIsNone(get_value(obj, "constructor"))
        self.assertIsNone(get_value(obj, "constructor.prototype"))
        self.assertIsNone(get_value(obj, "a.constructor"))

    def test_allow_normal_non_dangerous_keys(self):
        obj = {"proto": "safe", "construct": "ok", "proto_type": "fine"}
        self.assertEqual(get_value(obj, "proto"), "safe")
        self.assertEqual(get_value(obj, "construct"), "ok")
        self.assertEqual(get_value(obj, "proto_type"), "fine")


class TestGetValueDataTypes(unittest.TestCase):
    def test_preserve_original_data_types(self):
        obj = {
            "numbers": [0, 1, -1, 3.14],
            "booleans": [True, False],
            "objects": [{"key": "value"}],
            "mixed": [None, "", 0, False],
        }
        self.assertEqual(get_value(obj, "numbers[0]"), 0)
        self.assertEqual(get_value(obj, "numbers[2]"), -1)
        self.assertEqual(get_value(obj, "numbers[3]"), 3.14)
        self.assertEqual(get_value(obj, "booleans[0]"), True)
        self.assertEqual(get_value(obj, "booleans[1]"), False)
        self.assertEqual(get_value(obj, "objects[0]"), {"key": "value"})
        self.assertIsNone(get_value(obj, "mixed[0]"))
        self.assertEqual(get_value(obj, "mixed[1]"), "")
        self.assertEqual(get_value(obj, "mixed[2]"), 0)
        self.assertEqual(get_value(obj, "mixed[3]"), False)


if __name__ == "__main__":
    unittest.main()
