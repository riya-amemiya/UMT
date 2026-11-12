import unittest

from src.error import Error, Success, unwrap_or


class TestUnwrapOr(unittest.TestCase):
    def test_success_returns_value(self):
        """Test that unwrap_or returns the value of a Success."""
        self.assertEqual(unwrap_or(Success(value=5), 0), 5)

    def test_error_returns_default(self):
        """Test that unwrap_or returns the default for an Error."""
        self.assertEqual(unwrap_or(Error(error="fail"), 0), 0)

    def test_success_with_none_value_returns_none(self):
        """Test that a Success holding None is not replaced by the default."""
        self.assertIsNone(unwrap_or(Success(value=None), 7))

    def test_default_of_different_type(self):
        """Test that the default may have a different type than the value."""
        self.assertEqual(unwrap_or(Error(error=ValueError("fail")), "none"), "none")


if __name__ == "__main__":
    unittest.main()
