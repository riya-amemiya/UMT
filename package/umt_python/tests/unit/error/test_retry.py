import asyncio
import unittest

from src.error import retry


class TestRetry(unittest.TestCase):
    def test_successful_function(self):
        """Test that a successful async function returns its result."""

        async def successful_fn():
            return "success"

        result = asyncio.run(retry(successful_fn))
        self.assertEqual(result, "success")

    def test_retry_on_failure_then_success(self):
        """Test that retry works when function fails initially then succeeds."""
        attempts = [0]

        async def fail_then_succeed():
            attempts[0] += 1
            if attempts[0] < 3:
                raise ValueError("Not yet!")
            return "finally succeeded"

        result = asyncio.run(retry(fail_then_succeed, retries=3, delay=0.01))
        self.assertEqual(result, "finally succeeded")
        self.assertEqual(attempts[0], 3)

    def test_all_retries_exhausted(self):
        """Test that exception is raised when all retries fail."""

        async def always_fail():
            raise ValueError("Always fails")

        with self.assertRaises(ValueError) as context:
            asyncio.run(retry(always_fail, retries=2, delay=0.01))
        self.assertEqual(str(context.exception), "Always fails")

    def test_should_retry_returns_false(self):
        """Test that retry stops when should_retry returns False."""
        attempts = [0]

        async def always_fail():
            attempts[0] += 1
            raise ValueError("Custom error")

        def never_retry(_):
            return False

        with self.assertRaises(ValueError):
            asyncio.run(
                retry(always_fail, retries=5, delay=0.01, should_retry=never_retry)
            )
        self.assertEqual(attempts[0], 1)

    def test_custom_should_retry(self):
        """Test custom should_retry function that filters errors."""
        attempts = [0]

        async def fail_with_value_error():
            attempts[0] += 1
            if attempts[0] < 3:
                raise ValueError("Retryable")
            return "success"

        def retry_only_value_error(error):
            return isinstance(error, ValueError)

        result = asyncio.run(
            retry(
                fail_with_value_error,
                retries=5,
                delay=0.01,
                should_retry=retry_only_value_error,
            )
        )
        self.assertEqual(result, "success")

    def test_default_always_retry(self):
        """Test that default should_retry (None) retries on all errors."""
        attempts = [0]

        async def fail_with_different_errors():
            attempts[0] += 1
            if attempts[0] == 1:
                raise ValueError("First error")
            if attempts[0] == 2:
                raise TypeError("Second error")
            return "done"

        result = asyncio.run(retry(fail_with_different_errors, retries=3, delay=0.01))
        self.assertEqual(result, "done")
        self.assertEqual(attempts[0], 3)

    def test_zero_retries(self):
        """Test with zero retries - should still attempt once."""

        async def fail_once():
            raise ValueError("Fail")

        with self.assertRaises(ValueError):
            asyncio.run(retry(fail_once, retries=0, delay=0.01))


if __name__ == "__main__":
    unittest.main()
