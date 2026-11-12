import asyncio
import unittest

from src.async_util import map_series


class TestMapSeries(unittest.IsolatedAsyncioTestCase):
    async def test_processes_all_items_and_returns_results_in_order(self):
        async def times_ten(n, _index):
            return n * 10

        results = await map_series([1, 2, 3, 4, 5], times_ten)
        self.assertEqual(results, [10, 20, 30, 40, 50])

    async def test_runs_tasks_strictly_sequentially(self):
        state = {"running": 0, "max": 0}
        order = []

        async def task(n, _index):
            state["running"] += 1
            state["max"] = max(state["max"], state["running"])
            order.append(n)
            await asyncio.sleep(0.005)
            state["running"] -= 1
            return n

        await map_series([1, 2, 3, 4], task)
        self.assertEqual(state["max"], 1)
        self.assertEqual(order, [1, 2, 3, 4])

    async def test_handles_empty_list(self):
        async def double(n, _index):
            return n * 2

        results = await map_series([], double)
        self.assertEqual(results, [])

    async def test_provides_index_to_the_function(self):
        async def label(item, index):
            return f"{item}-{index}"

        results = await map_series(["a", "b", "c"], label)
        self.assertEqual(results, ["a-0", "b-1", "c-2"])

    async def test_raises_when_any_task_fails_and_stops(self):
        seen = []

        async def task(n, _index):
            seen.append(n)
            if n == 2:
                raise ValueError("fail")
            return n

        with self.assertRaises(ValueError) as context:
            await map_series([1, 2, 3], task)
        self.assertEqual(str(context.exception), "fail")
        self.assertEqual(seen, [1, 2])

    async def test_handles_single_element_list(self):
        async def inc(n, _index):
            return n + 1

        results = await map_series([7], inc)
        self.assertEqual(results, [8])


if __name__ == "__main__":
    unittest.main()
