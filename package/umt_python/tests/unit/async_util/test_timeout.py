import asyncio
import unittest

from src.async_util import timeout


class TestTimeout(unittest.IsolatedAsyncioTestCase):
    async def test_resolves_when_completes_before_timeout(self):
        async def quick():
            await asyncio.sleep(0.01)
            return "done"

        result = await timeout(quick(), 1.0)
        self.assertEqual(result, "done")

    async def test_raises_timeout_error_when_exceeds_time(self):
        async def slow():
            await asyncio.sleep(10)
            return "done"

        with self.assertRaises(TimeoutError) as ctx:
            await timeout(slow(), 0.02)
        self.assertEqual(str(ctx.exception), "Timed out after 0.02s")

    async def test_raises_original_error_when_promise_fails(self):
        async def boom():
            await asyncio.sleep(0.01)
            raise Exception("original error")

        with self.assertRaises(Exception) as ctx:
            await timeout(boom(), 1.0)
        self.assertEqual(str(ctx.exception), "original error")

    async def test_resolves_already_completed(self):
        async def immediate():
            return 42

        result = await timeout(immediate(), 1.0)
        self.assertEqual(result, 42)


if __name__ == "__main__":
    unittest.main()
