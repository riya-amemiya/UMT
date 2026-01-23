import unittest

from src.error import safe_execute, Success, Error


class TestSafeExecute(unittest.TestCase):
    def test_successful_execution(self):
        """Test that successful callback returns Success result."""
        result = safe_execute(lambda: 1 + 1)
        self.assertIsInstance(result, Success)
        self.assertEqual(result.type, "success")
        self.assertEqual(result.value, 2)

    def test_execution_with_exception(self):
        """Test that exception is caught and wrapped in Error result."""
        result = safe_execute(lambda: 1 / 0)
        self.assertIsInstance(result, Error)
        self.assertEqual(result.type, "error")
        self.assertIsInstance(result.error, ZeroDivisionError)

    def test_execution_with_value_error(self):
        """Test catching ValueError."""

        def raise_value_error():
            raise ValueError("Test error")

        result = safe_execute(raise_value_error)
        self.assertIsInstance(result, Error)
        self.assertEqual(result.type, "error")
        self.assertIsInstance(result.error, ValueError)
        self.assertEqual(str(result.error), "Test error")

    def test_execution_returns_none(self):
        """Test that returning None is treated as success."""
        result = safe_execute(lambda: None)
        self.assertIsInstance(result, Success)
        self.assertEqual(result.type, "success")
        self.assertIsNone(result.value)

    def test_docstring_example(self):
        """Test examples from docstring."""
        result = safe_execute(lambda: 1 / 0)
        self.assertEqual(result.type, "error")

        result = safe_execute(lambda: 1 + 1)
        self.assertEqual(result.value, 2)


if __name__ == "__main__":
    unittest.main()
