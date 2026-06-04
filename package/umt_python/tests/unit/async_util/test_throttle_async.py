import asyncio
import unittest

from src.async_util import sleep, throttle_async


class TestThrottleAsync(unittest.IsolatedAsyncioTestCase):
    async def test_returns_the_same_inflight_to_concurrent_callers(self):
        state = {"calls": 0}

        async def fn(value):
            state["calls"] += 1
            await sleep(0.01)
            return value

        throttled = throttle_async(fn, 0)
        a = throttled(1)
        b = throttled(2)
        self.assertIs(a, b)
        await asyncio.gather(a, b)
        self.assertEqual(state["calls"], 1)

    async def test_rejects_within_lockout_window_after_completion(self):
        state = {"calls": 0}

        async def fn(value):
            state["calls"] += 1
            return value

        throttled = throttle_async(fn, 1.0)
        await throttled(1)
        with self.assertRaises(Exception) as ctx:
            await throttled(2)
        self.assertEqual(str(ctx.exception), "throttleAsync window not elapsed")
        self.assertEqual(state["calls"], 1)

    async def test_allows_a_new_invocation_after_the_wait_window(self):
        state = {"calls": 0}

        async def fn(value):
            state["calls"] += 1
            return value

        throttled = throttle_async(fn, 0.005)
        await throttled(1)
        await sleep(0.015)
        await throttled(2)
        self.assertEqual(state["calls"], 2)

    async def test_cancel_clears_lockout_allowing_new_invocation(self):
        state = {"calls": 0}

        async def fn(value):
            state["calls"] += 1
            return value

        throttled = throttle_async(fn, 1.0)
        await throttled(1)
        throttled.cancel()
        await throttled(2)
        self.assertEqual(state["calls"], 2)

    async def test_propagates_rejection(self):
        async def fn():
            raise Exception("boom")

        throttled = throttle_async(fn, 0.005)
        with self.assertRaises(Exception) as ctx:
            await throttled()
        self.assertEqual(str(ctx.exception), "boom")

    async def test_does_not_clear_newer_inflight_when_older_resolves_after_cancel(
        self,
    ):
        state = {"calls": 0}
        futures = {}

        async def fn(value):
            state["calls"] += 1
            fut = asyncio.get_event_loop().create_future()
            futures[value] = fut
            return await fut

        throttled = throttle_async(fn, 1.0)
        first = throttled(1)
        throttled.cancel()
        second = throttled(2)
        # Let both calls start so their futures register.
        await asyncio.sleep(0)
        futures[1].set_result(1)
        await first
        futures[2].set_result(2)
        await second
        self.assertEqual(state["calls"], 2)


if __name__ == "__main__":
    unittest.main()
