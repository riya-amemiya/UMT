import asyncio
import unittest

from src.async_util import parallel


class TestParallel(unittest.IsolatedAsyncioTestCase):
    async def test_processes_all_items_in_order(self):
        items = [1, 2, 3, 4, 5]

        async def times_ten(n, _index):
            return n * 10

        results = await parallel(2, items, times_ten)
        self.assertEqual(results, [10, 20, 30, 40, 50])

    async def test_respects_the_concurrency_limit(self):
        state = {"running": 0, "max": 0}
        items = [1, 2, 3, 4, 5, 6]

        async def task(n, _index):
            state["running"] += 1
            state["max"] = max(state["max"], state["running"])
            await asyncio.sleep(0.01)
            state["running"] -= 1
            return n

        await parallel(3, items, task)
        self.assertLessEqual(state["max"], 3)

    async def test_handles_empty_list(self):
        async def double(n, _index):
            return n * 2

        results = await parallel(2, [], double)
        self.assertEqual(results, [])

    async def test_handles_limit_larger_than_items_length(self):
        async def inc(n, _index):
            return n + 1

        results = await parallel(10, [1, 2], inc)
        self.assertEqual(results, [2, 3])

    async def test_raises_when_any_task_fails(self):
        async def task(n, _index):
            if n == 2:
                raise Exception("fail")
            return n

        with self.assertRaises(Exception) as ctx:
            await parallel(2, [1, 2, 3], task)
        self.assertEqual(str(ctx.exception), "fail")

    async def test_provides_index_to_the_function(self):
        items = ["a", "b", "c"]

        async def fmt(item, index):
            return f"{item}-{index}"

        results = await parallel(2, items, fmt)
        self.assertEqual(results, ["a-0", "b-1", "c-2"])

    async def test_handles_limit_of_one_sequential(self):
        order: list[int] = []

        async def task(n, _index):
            order.append(n)
            return n

        await parallel(1, [1, 2, 3], task)
        self.assertEqual(order, [1, 2, 3])


if __name__ == "__main__":
    unittest.main()
