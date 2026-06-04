import asyncio
import unittest

from src.async_util import debounce_async, sleep


class TestDebounceAsync(unittest.IsolatedAsyncioTestCase):
    async def test_invokes_once_for_rapid_calls_with_latest_result(self):
        state = {"calls": 0, "last_arg": None}

        async def fn(value):
            state["calls"] += 1
            state["last_arg"] = value
            return value * 2

        debounced = debounce_async(fn, 0.01)
        results = await asyncio.gather(debounced(1), debounced(2), debounced(3))
        self.assertEqual(state["calls"], 1)
        self.assertEqual(state["last_arg"], 3)
        self.assertEqual(results, [6, 6, 6])

    async def test_invokes_again_after_the_wait_elapsed(self):
        state = {"calls": 0}

        async def fn(value):
            state["calls"] += 1
            return value

        debounced = debounce_async(fn, 0.005)
        await debounced(1)
        await sleep(0.01)
        await debounced(2)
        self.assertEqual(state["calls"], 2)

    async def test_rejects_all_pending_callers_when_cancelled(self):
        async def fn(value):
            return value

        debounced = debounce_async(fn, 0.05)
        pending = debounced(1)
        debounced.cancel()
        with self.assertRaises(Exception) as ctx:
            await pending
        self.assertEqual(str(ctx.exception), "debounceAsync cancelled")

    async def test_propagates_rejection_from_underlying_function(self):
        async def fn():
            raise Exception("boom")

        debounced = debounce_async(fn, 0.005)
        with self.assertRaises(Exception) as ctx:
            await debounced()
        self.assertEqual(str(ctx.exception), "boom")

    async def test_cancel_is_a_noop_when_nothing_pending(self):
        async def fn():
            return 1

        debounced = debounce_async(fn, 0.01)
        # Should not raise.
        debounced.cancel()


if __name__ == "__main__":
    unittest.main()
