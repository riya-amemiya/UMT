import asyncio
import unittest

from src.async_util import memoize_async


class TestMemoizeAsync(unittest.IsolatedAsyncioTestCase):
    async def test_caches_resolved_values_by_first_argument(self):
        calls = []

        async def double(n):
            calls.append(n)
            return n * 2

        memoized = memoize_async(double)
        self.assertEqual(await memoized(5), 10)
        self.assertEqual(await memoized(5), 10)
        self.assertEqual(await memoized(6), 12)
        self.assertEqual(calls, [5, 6])
        self.assertEqual(memoized.cache[5], 10)
        self.assertEqual(len(memoized.cache), 2)

    async def test_shares_one_in_flight_call_for_the_same_key(self):
        calls = []
        gate = asyncio.Event()

        async def fn(key):
            calls.append(key)
            await gate.wait()
            return "done"

        memoized = memoize_async(fn)
        first = asyncio.ensure_future(memoized("a"))
        second = asyncio.ensure_future(memoized("a"))
        await asyncio.sleep(0)
        self.assertEqual(len(memoized.cache), 0)
        gate.set()
        self.assertEqual(await asyncio.gather(first, second), ["done", "done"])
        self.assertEqual(calls, ["a"])
        self.assertEqual(memoized.cache["a"], "done")

    async def test_does_not_cache_raised_errors(self):
        state = {"calls": 0}
        gate = asyncio.Event()

        async def fn(n):
            state["calls"] += 1
            if state["calls"] == 1:
                await gate.wait()
                raise ValueError("fail")
            return 42

        memoized = memoize_async(fn)
        first = asyncio.ensure_future(memoized(1))
        second = asyncio.ensure_future(memoized(1))
        await asyncio.sleep(0)
        gate.set()
        results = await asyncio.gather(first, second, return_exceptions=True)
        self.assertTrue(all(isinstance(result, ValueError) for result in results))
        self.assertNotIn(1, memoized.cache)
        self.assertEqual(await memoized(1), 42)
        self.assertEqual(state["calls"], 2)

    async def test_invokes_again_right_after_an_error(self):
        state = {"calls": 0}

        async def fn(key):
            state["calls"] += 1
            if state["calls"] == 1:
                raise ValueError("fail")
            return "ok"

        memoized = memoize_async(fn)
        with self.assertRaises(ValueError):
            await memoized("k")
        self.assertEqual(await memoized("k"), "ok")
        self.assertEqual(state["calls"], 2)

    async def test_caches_none_values(self):
        calls = []

        async def fn(key):
            calls.append(key)

        memoized = memoize_async(fn)
        await memoized("x")
        self.assertIsNone(await memoized("x"))
        self.assertEqual(calls, ["x"])

    async def test_no_arguments_use_none_key(self):
        calls = []

        async def fn():
            calls.append(1)
            return "value"

        memoized = memoize_async(fn)
        self.assertEqual(await memoized(), "value")
        self.assertEqual(await memoized(), "value")
        self.assertEqual(calls, [1])
        self.assertIn(None, memoized.cache)

    async def test_custom_resolver(self):
        calls = []

        async def add(a, b):
            calls.append((a, b))
            return a + b

        memoized = memoize_async(add, resolver=lambda a, b: f"{a},{b}")
        self.assertEqual(await memoized(1, 2), 3)
        self.assertEqual(await memoized(1, 2), 3)
        self.assertEqual(await memoized(1, 3), 4)
        self.assertEqual(len(calls), 2)
        self.assertEqual(list(memoized.cache.keys()), ["1,2", "1,3"])

    async def test_evicts_oldest_entry_at_max_size(self):
        calls = []

        async def identity(n):
            calls.append(n)
            return n

        memoized = memoize_async(identity, max_size=2)
        await memoized(1)
        await memoized(2)
        await memoized(3)
        self.assertEqual(list(memoized.cache.keys()), [2, 3])
        await memoized(1)
        self.assertEqual(len(calls), 4)


if __name__ == "__main__":
    unittest.main()
