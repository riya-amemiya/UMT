import time
import unittest

from src.async_util import sleep


class TestSleep(unittest.IsolatedAsyncioTestCase):
    async def test_resolves_after_the_specified_seconds(self):
        result = await sleep(0.01)
        self.assertIsNone(result)

    async def test_does_not_resolve_before_the_specified_time(self):
        start = time.monotonic()
        await sleep(0.05)
        elapsed = time.monotonic() - start
        self.assertGreaterEqual(elapsed, 0.04)

    async def test_resolves_with_zero_delay(self):
        result = await sleep(0)
        self.assertIsNone(result)


if __name__ == "__main__":
    unittest.main()
