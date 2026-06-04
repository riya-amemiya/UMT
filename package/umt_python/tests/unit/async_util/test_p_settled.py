import asyncio
import unittest

from src.async_util import p_settled


class TestPSettled(unittest.IsolatedAsyncioTestCase):
    async def test_resolves_with_mixed_fulfilled_and_rejected(self):
        async def ok():
            return 1

        async def bad():
            raise Exception("nope")

        result = await p_settled([ok(), bad()])
        self.assertEqual(result[0].status, "fulfilled")
        self.assertEqual(result[0].value, 1)
        self.assertEqual(result[1].status, "rejected")
        self.assertEqual(str(result[1].reason), "nope")

    async def test_returns_empty_list_for_empty_input(self):
        self.assertEqual(await p_settled([]), [])

    async def test_supports_thunks(self):
        state = {"started": 0}

        def make(value):
            async def thunk():
                state["started"] += 1
                return value

            return thunk

        result = await p_settled([make(1), make(2)], 1)
        self.assertEqual(state["started"], 2)
        self.assertEqual(
            [r.value if r.status == "fulfilled" else None for r in result],
            [1, 2],
        )

    async def test_respects_concurrency_limit(self):
        state = {"active": 0, "peak": 0}

        def make(index):
            async def thunk():
                state["active"] += 1
                state["peak"] = max(state["peak"], state["active"])
                await asyncio.sleep(0.005)
                state["active"] -= 1
                return index

            return thunk

        tasks = [make(i) for i in range(6)]
        await p_settled(tasks, 2)
        self.assertLessEqual(state["peak"], 2)

    async def test_preserves_input_order(self):
        async def slow():
            await asyncio.sleep(0.02)
            return 1

        async def fast():
            return 2

        result = await p_settled([slow(), fast()])
        self.assertEqual(
            [r.value if r.status == "fulfilled" else None for r in result],
            [1, 2],
        )

    async def test_treats_limit_of_zero_as_unlimited(self):
        async def one():
            return 1

        async def two():
            return 2

        result = await p_settled([one(), two()], 0)
        self.assertEqual(len(result), 2)


if __name__ == "__main__":
    unittest.main()
