import asyncio
import unittest

from src.async_util import defer


class TestDefer(unittest.IsolatedAsyncioTestCase):
    async def test_resolves_with_the_given_value(self):
        d = defer()
        d.resolve(42)
        result = await d.future
        self.assertEqual(result, 42)

    async def test_rejects_with_the_given_reason(self):
        d = defer()
        d.reject(Exception("deferred error"))
        with self.assertRaises(Exception) as ctx:
            await d.future
        self.assertEqual(str(ctx.exception), "deferred error")

    async def test_exposes_resolve_and_reject_as_callables(self):
        d = defer()
        self.assertTrue(callable(d.resolve))
        self.assertTrue(callable(d.reject))
        self.assertIsInstance(d.future, asyncio.Future)

    async def test_works_with_async_await_pattern(self):
        d = defer()
        loop = asyncio.get_event_loop()
        loop.call_later(0.01, lambda: d.resolve("delayed"))
        result = await d.future
        self.assertEqual(result, "delayed")

    async def test_reject_with_non_exception_reason_wraps_in_exception(self):
        d = defer()
        d.reject("string reason")
        with self.assertRaises(Exception) as ctx:
            await d.future
        self.assertEqual(str(ctx.exception), "string reason")


if __name__ == "__main__":
    unittest.main()
