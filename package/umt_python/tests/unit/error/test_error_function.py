import unittest

from src.error import Error, error_function


class TestErrorFunction(unittest.TestCase):
    def test_create_error_result(self):
        """Test that error_function creates an error result."""
        result = error_function(ValueError("test"))
        self.assertIsInstance(result, Error)
        self.assertEqual(result.type, "error")
        self.assertIsInstance(result.error, ValueError)

    def test_preserves_error_message(self):
        """Test that the original error message is preserved."""
        result = error_function(TypeError("custom type error"))
        self.assertEqual(result.type, "error")
        self.assertIsInstance(result.error, TypeError)
        self.assertEqual(str(result.error), "custom type error")

    def test_non_exception_error_value(self):
        """Test that non-exception error values are wrapped unchanged."""
        result = error_function("string error")
        self.assertEqual(result.type, "error")
        self.assertEqual(result.error, "string error")


if __name__ == "__main__":
    unittest.main()
