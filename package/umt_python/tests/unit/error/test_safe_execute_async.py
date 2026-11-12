import unittest

from src.error import Error, Success, safe_execute_async


class TestSafeExecuteAsync(unittest.IsolatedAsyncioTestCase):
    async def test_returns_success_for_an_async_callback(self):
        async def answer():
            return 42

        result = await safe_execute_async(answer)
        self.assertIsInstance(result, Success)
        self.assertEqual(result.type, "success")
        self.assertEqual(result.value, 42)

    async def test_returns_success_for_a_sync_return_value(self):
        result = await safe_execute_async(lambda: "ok")
        self.assertIsInstance(result, Success)
        self.assertEqual(result.value, "ok")

    async def test_returns_success_with_object_value(self):
        value = {"key": "value"}

        async def get_value():
            return value

        result = await safe_execute_async(get_value)
        self.assertIs(result.value, value)

    async def test_returns_error_for_a_failing_async_callback(self):
        error = RuntimeError("async fail")

        async def fail():
            raise error

        result = await safe_execute_async(fail)
        self.assertIsInstance(result, Error)
        self.assertEqual(result.type, "error")
        self.assertIs(result.error, error)
        self.assertEqual(str(result.error), "async fail")

    async def test_returns_error_for_a_sync_raise(self):
        error = TypeError("sync fail")

        def fail():
            raise error

        result = await safe_execute_async(fail)
        self.assertIsInstance(result, Error)
        self.assertIs(result.error, error)


if __name__ == "__main__":
    unittest.main()
