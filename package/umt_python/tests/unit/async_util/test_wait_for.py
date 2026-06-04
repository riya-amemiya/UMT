import asyncio
import unittest

from src.async_util import AbortSignal, wait_for


class TestWaitFor(unittest.IsolatedAsyncioTestCase):
    async def test_resolves_immediately_when_initially_truthy(self):
        result = await wait_for(lambda: 42)
        self.assertEqual(result, 42)

    async def test_resolves_once_condition_becomes_truthy(self):
        state = {"counter": 0}

        def condition():
            state["counter"] += 1
            return "done" if state["counter"] >= 3 else None

        result = await wait_for(condition, interval=0.001, timeout=0.5)
        self.assertEqual(result, "done")

    async def test_rejects_on_timeout(self):
        with self.assertRaises(TimeoutError) as ctx:
            await wait_for(lambda: None, interval=0.005, timeout=0.02)
        self.assertIn("timed out", str(ctx.exception))

    async def test_supports_async_predicate(self):
        state = {"counter": 0}

        async def condition():
            state["counter"] += 1
            return state["counter"] if state["counter"] >= 2 else 0

        result = await wait_for(condition, interval=0.001, timeout=0.5)
        self.assertEqual(result, 2)

    async def test_rejects_when_signal_already_aborted(self):
        signal = AbortSignal()
        signal.abort(Exception("nope"))
        with self.assertRaises(Exception) as ctx:
            await wait_for(lambda: 1, signal=signal)
        self.assertEqual(str(ctx.exception), "nope")

    async def test_rejects_when_condition_throws(self):
        def condition():
            raise Exception("boom")

        with self.assertRaises(Exception) as ctx:
            await wait_for(condition, timeout=0.1)
        self.assertEqual(str(ctx.exception), "boom")

    async def test_aborts_during_polling(self):
        signal = AbortSignal()
        loop = asyncio.get_event_loop()
        loop.call_later(0.005, lambda: signal.abort(Exception("stopped")))
        with self.assertRaises(Exception) as ctx:
            await wait_for(
                lambda: None, signal=signal, interval=0.002, timeout=1.0
            )
        self.assertEqual(str(ctx.exception), "stopped")

    async def test_aborts_without_explicit_reason(self):
        signal = AbortSignal()
        loop = asyncio.get_event_loop()
        loop.call_later(0.005, signal.abort)
        with self.assertRaises(Exception):
            await wait_for(
                lambda: None, signal=signal, interval=0.05, timeout=1.0
            )

    async def test_falls_back_to_generic_aborted_error(self):
        signal = AbortSignal()
        signal.aborted = True
        with self.assertRaises(Exception) as ctx:
            await wait_for(lambda: None, signal=signal)
        self.assertEqual(str(ctx.exception), "Aborted")


if __name__ == "__main__":
    unittest.main()
