import unittest

from src.error import Error, Success, match_result


class TestMatchResult(unittest.TestCase):
    def test_success_calls_on_success(self):
        """Test that match_result calls on_success for a Success result."""
        result = Success(value=42)
        output = match_result(result, lambda v: f"Got {v}", lambda e: f"Failed: {e}")
        self.assertEqual(output, "Got 42")

    def test_error_calls_on_error(self):
        """Test that match_result calls on_error for an Error result."""
        result = Error(error=ValueError("oops"))
        output = match_result(result, lambda v: f"Got {v}", lambda e: f"Failed: {e}")
        self.assertEqual(output, "Failed: oops")

    def test_success_returns_computed_value(self):
        """Test that match_result returns the computed value from on_success."""
        result = Success(value=10)
        output = match_result(result, lambda v: v * 2, lambda _e: -1)
        self.assertEqual(output, 20)

    def test_error_returns_computed_value(self):
        """Test that match_result returns the computed value from on_error."""
        result = Error(error=ValueError("fail"))
        output = match_result(result, lambda v: v * 2, lambda _e: -1)  # type: ignore[reportOperatorIssue]
        self.assertEqual(output, -1)


if __name__ == "__main__":
    unittest.main()
