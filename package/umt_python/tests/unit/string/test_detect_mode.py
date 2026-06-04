import unittest

from src.string import detect_mode
from src.string.detect_mode import _UNDEFINED


class TestDetectModeNamed(unittest.TestCase):
    def test_object_data_only(self):
        result = detect_mode({"name": "Alice", "age": 30})
        self.assertEqual(result["data"], {"name": "Alice", "age": 30})
        self.assertEqual(result["options"], {})

    def test_object_data_and_options(self):
        data = {"name": "Bob", "score": 95}
        options = {"formatters": {"custom": lambda v, *_: "test"}}
        result = detect_mode(data, options)
        self.assertIs(result["data"], data)
        self.assertIs(result["options"], options)

    def test_empty_object_is_named(self):
        result = detect_mode({})
        self.assertEqual(result["data"], {})
        self.assertEqual(result["options"], {})

    def test_complex_nested_objects(self):
        complex_data = {
            "user": {"name": "Alice", "profile": {"age": 25}},
            "items": ["item1", "item2"],
        }
        result = detect_mode(complex_data)
        self.assertIs(result["data"], complex_data)
        self.assertEqual(result["options"], {})


class TestDetectModeIndexed(unittest.TestCase):
    def test_multiple_primitive_values(self):
        result = detect_mode("first", "second", ["third", "fourth"])
        self.assertEqual(result["data"], ["first", "second", "third", "fourth"])
        self.assertEqual(result["options"], {})

    def test_single_value(self):
        result = detect_mode("single")
        self.assertEqual(result["data"], ["single"])
        self.assertEqual(result["options"], {})

    def test_mixed_types(self):
        result = detect_mode("string", 42, [True, None, {"key": "value"}])
        self.assertEqual(result["data"], ["string", 42, True, None, {"key": "value"}])
        self.assertEqual(result["options"], {})

    def test_undefined_as_first_value(self):
        result = detect_mode(_UNDEFINED, "second", ["third"])
        self.assertEqual(result["data"], ["second", "third"])
        self.assertEqual(result["options"], {})


class TestDetectModeObjectDifferentiation(unittest.TestCase):
    def test_arrays_are_indexed(self):
        result = detect_mode(["array", "values"], "second")
        self.assertEqual(result["data"], [["array", "values"], "second"])
        self.assertEqual(result["options"], {})

    def test_null_is_indexed(self):
        result = detect_mode(None, "second")
        self.assertEqual(result["data"], [None, "second"])
        self.assertEqual(result["options"], {})

    def test_differentiate_data_object_and_options(self):
        data_object = {"name": "Alice"}
        options_object = {"formatters": {"upper": lambda s, *_: str(s).upper()}}
        result = detect_mode(data_object, options_object)
        self.assertIs(result["data"], data_object)
        self.assertIs(result["options"], options_object)


class TestDetectModeEdgeCases(unittest.TestCase):
    def test_all_undefined(self):
        result = detect_mode(_UNDEFINED, _UNDEFINED, [])
        self.assertEqual(result["data"], [])
        self.assertEqual(result["options"], {})

    def test_object_with_additional_values_is_indexed(self):
        result = detect_mode({"name": "Alice"}, "extra", ["more"])
        self.assertEqual(result["data"], [{"name": "Alice"}, "extra", "more"])
        self.assertEqual(result["options"], {})

    def test_non_options_object_as_second_parameter(self):
        result = detect_mode({"name": "Alice"}, {"notFormatters": True})
        self.assertEqual(result["data"], [{"name": "Alice"}, {"notFormatters": True}])
        self.assertEqual(result["options"], {})

    def test_primitive_values_as_first_argument(self):
        self.assertEqual(detect_mode("string")["data"], ["string"])
        self.assertEqual(detect_mode(42)["data"], [42])
        self.assertEqual(detect_mode(True)["data"], [True])
        self.assertEqual(detect_mode(False)["data"], [False])


class TestDetectModeOptionsValidation(unittest.TestCase):
    def test_recognize_valid_options(self):
        options = {
            "formatters": {
                "custom": lambda value, *_: str(value),
                "upper": lambda s, *_: str(s).upper(),
            }
        }
        result = detect_mode({"name": "test"}, options)
        self.assertEqual(result["data"], {"name": "test"})
        self.assertIs(result["options"], options)

    def test_reject_object_without_formatters(self):
        not_options = {"someOtherProperty": "value"}
        result = detect_mode({"name": "test"}, not_options)
        self.assertEqual(result["data"], [{"name": "test"}, not_options])
        self.assertEqual(result["options"], {})

    def test_reject_array_as_options(self):
        result = detect_mode({"name": "test"}, ["not", "options"])
        self.assertEqual(result["data"], [{"name": "test"}, ["not", "options"]])
        self.assertEqual(result["options"], {})


class TestDetectModeComplex(unittest.TestCase):
    def test_preserve_object_references(self):
        original_object = {"shared": "reference"}
        result = detect_mode(original_object)
        self.assertIs(result["data"], original_object)

    def test_mixed_scenarios(self):
        obj = {"key": "value"}
        result = detect_mode(obj, "string", [42, True])
        self.assertEqual(result["data"], [obj, "string", 42, True])
        self.assertEqual(result["options"], {})


if __name__ == "__main__":
    unittest.main()
