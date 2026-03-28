import unittest

from src.error import Error, Success, map_result


class TestMapResult(unittest.TestCase):
    def test_success_transforms_value(self):
        """Test that map_result transforms the value inside a Success."""
        result = map_result(Success(value=5), lambda n: n * 2)
        self.assertIsInstance(result, Success)
        self.assertEqual(result.value, 10)

    def test_success_changes_type(self):
        """Test that map_result can change the value type."""
        result = map_result(Success(value=42), lambda n: f"value: {n}")
        self.assertIsInstance(result, Success)
        self.assertEqual(result.value, "value: 42")

    def test_error_returns_unchanged(self):
        """Test that map_result returns the error unchanged."""
        error = Error(error=ValueError("fail"))
        result = map_result(error, lambda n: n * 2)  # type: ignore[reportOperatorIssue]
        self.assertIsInstance(result, Error)
        self.assertEqual(result.type, "error")
        self.assertIsInstance(result.error, ValueError)

    def test_success_with_list(self):
        """Test mapping over a list value."""
        result = map_result(Success(value=[1, 2, 3]), lambda v: len(v))
        self.assertIsInstance(result, Success)
        self.assertEqual(result.value, 3)


if __name__ == "__main__":
    unittest.main()
