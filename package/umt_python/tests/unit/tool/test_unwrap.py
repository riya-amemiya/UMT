import unittest

from src.tool.unwrap import unwrap


class TestUnwrap(unittest.TestCase):
    def test_should_return_the_value_when_it_is_not_undefined_or_null(self):
        value = "test"
        result = unwrap(value, "Value is absent")
        self.assertEqual(result, "test")

    def test_should_return_the_value_when_it_is_a_number(self):
        value = 42
        result = unwrap(value, "Value is absent")
        self.assertEqual(result, 42)

    def test_should_return_the_value_when_it_is_zero(self):
        value = 0
        result = unwrap(value, "Value is absent")
        self.assertEqual(result, 0)

    def test_should_return_the_value_when_it_is_an_empty_string(self):
        value = ""
        result = unwrap(value, "Value is absent")
        self.assertEqual(result, "")

    def test_should_return_the_value_when_it_is_false(self):
        value = False
        result = unwrap(value, "Value is absent")
        self.assertEqual(result, False)

    def test_should_return_the_value_when_it_is_an_object(self):
        value = {"key": "value"}
        result = unwrap(value, "Value is absent")
        self.assertEqual(result, {"key": "value"})

    def test_should_return_the_value_when_it_is_an_array(self):
        value = [1, 2, 3]
        result = unwrap(value, "Value is absent")
        self.assertEqual(result, [1, 2, 3])

    def test_should_throw_an_error_when_the_value_is_null(self):
        value = None
        with self.assertRaisesRegex(ValueError, "Value is null"):
            unwrap(value, "Value is null")


if __name__ == "__main__":
    unittest.main()
