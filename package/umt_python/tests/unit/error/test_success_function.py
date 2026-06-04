import unittest

from src.error import Success, success_function


class TestSuccessFunction(unittest.TestCase):
    def test_create_success_result(self):
        """Test that success_function creates a success result."""
        result = success_function(42)
        self.assertIsInstance(result, Success)
        self.assertEqual(result.type, "success")
        self.assertEqual(result.value, 42)

    def test_success_with_string(self):
        """Test that success_function wraps a string value."""
        result = success_function("test")
        self.assertEqual(result.type, "success")
        self.assertEqual(result.value, "test")

    def test_success_with_object(self):
        """Test that success_function wraps an object value unchanged."""
        test_object = {"key": "value"}
        result = success_function(test_object)
        self.assertEqual(result.type, "success")
        self.assertIs(result.value, test_object)

    def test_success_with_none(self):
        """Test that success_function wraps None as a success value."""
        result = success_function(None)
        self.assertEqual(result.type, "success")
        self.assertIsNone(result.value)


if __name__ == "__main__":
    unittest.main()
