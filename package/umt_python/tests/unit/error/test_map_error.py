import unittest

from src.error import Error, Success, map_error


class TestMapError(unittest.TestCase):
    def test_error_transforms_error(self):
        """Test that map_error transforms the error inside an Error."""
        result = map_error(Error(error="fail"), lambda message: f"wrapped: {message}")
        self.assertIsInstance(result, Error)
        self.assertEqual(result.error, "wrapped: fail")

    def test_success_returns_unchanged(self):
        """Test that map_error returns a Success unchanged without calling f."""
        calls = []
        success = Success(value=5)
        result = map_error(success, lambda message: calls.append(message))
        self.assertIs(result, success)
        self.assertEqual(calls, [])

    def test_error_changes_type(self):
        """Test that map_error can change the error type."""
        result = map_error(Error(error="boom"), ValueError)
        self.assertIsInstance(result.error, ValueError)
        self.assertEqual(str(result.error), "boom")


if __name__ == "__main__":
    unittest.main()
