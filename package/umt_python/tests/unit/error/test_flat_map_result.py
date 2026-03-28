import unittest

from src.error import Error, Success, flat_map_result


class TestFlatMapResult(unittest.TestCase):
    def test_success_returns_new_success(self):
        """Test that flat_map_result returns a new Success from the mapping function."""
        result = flat_map_result(Success(value=5), lambda n: Success(value=n * 2))
        self.assertIsInstance(result, Success)
        self.assertEqual(result.value, 10)

    def test_success_returns_new_error(self):
        """Test that flat_map_result can return an Error from the mapping function."""
        result = flat_map_result(
            Success(value=-1),
            lambda n: Success(value=n * 2)
            if n > 0
            else Error(error=ValueError("negative")),
        )
        self.assertIsInstance(result, Error)
        self.assertIsInstance(result.error, ValueError)

    def test_error_returns_unchanged(self):
        """Test that flat_map_result returns the original error unchanged."""
        error = Error(error=ValueError("original error"))
        result = flat_map_result(error, lambda n: Success(value=n * 2))  # type: ignore[reportOperatorIssue]
        self.assertIsInstance(result, Error)
        self.assertEqual(str(result.error), "original error")

    def test_chaining(self):
        """Test chaining flat_map_result calls."""
        result = flat_map_result(
            Success(value=10),
            lambda n: Success(value=f"result: {n}"),
        )
        self.assertIsInstance(result, Success)
        self.assertEqual(result.value, "result: 10")


if __name__ == "__main__":
    unittest.main()
