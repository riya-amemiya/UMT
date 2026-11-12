import unittest
from unittest.mock import patch

from src.async_util import AbortSignal, retry
from src.error.retry import _compute_delay


def _failing(times, error_factory=lambda: ValueError("nope")):
    state = {"attempts": 0}

    async def fn():
        state["attempts"] += 1
        if state["attempts"] <= times:
            raise error_factory()
        return "ok"

    return fn, state


class TestComputeDelay(unittest.TestCase):
    def test_fixed_backoff_keeps_the_base_delay(self):
        self.assertEqual(_compute_delay(2.0, 1, "fixed", False), 2.0)
        self.assertEqual(_compute_delay(2.0, 4, "fixed", False), 2.0)

    def test_linear_backoff_scales_with_the_attempt_number(self):
        self.assertEqual(_compute_delay(2.0, 1, "linear", False), 2.0)
        self.assertEqual(_compute_delay(2.0, 3, "linear", False), 6.0)

    def test_exponential_backoff_doubles_each_attempt(self):
        self.assertEqual(_compute_delay(2.0, 1, "exponential", False), 2.0)
        self.assertEqual(_compute_delay(2.0, 2, "exponential", False), 4.0)
        self.assertEqual(_compute_delay(2.0, 4, "exponential", False), 16.0)

    def test_jitter_multiplies_by_a_random_factor(self):
        with patch("src.error.retry.random_module.random", return_value=0.25):
            self.assertEqual(_compute_delay(2.0, 3, "linear", True), 1.5)


class TestRetryOptions(unittest.IsolatedAsyncioTestCase):
    async def test_returns_the_first_successful_result(self):
        fn, state = _failing(0)
        self.assertEqual(await retry(fn, delay=0), "ok")
        self.assertEqual(state["attempts"], 1)

    async def test_on_retry_receives_error_and_attempt_number(self):
        fn, state = _failing(2)
        calls = []

        def on_retry(error, attempt):
            calls.append((str(error), attempt))

        result = await retry(fn, retries=3, delay=0, on_retry=on_retry)
        self.assertEqual(result, "ok")
        self.assertEqual(state["attempts"], 3)
        self.assertEqual(calls, [("nope", 0), ("nope", 1)])

    async def test_on_retry_is_not_called_when_budget_is_exhausted(self):
        fn, state = _failing(10)
        calls = []

        with self.assertRaises(ValueError):
            await retry(fn, retries=1, delay=0, on_retry=lambda _e, a: calls.append(a))
        self.assertEqual(state["attempts"], 2)
        self.assertEqual(calls, [0])

    async def test_should_retry_false_stops_before_on_retry(self):
        fn, state = _failing(10)
        calls = []

        with self.assertRaises(ValueError):
            await retry(
                fn,
                retries=5,
                delay=0,
                should_retry=lambda _e: False,
                on_retry=lambda _e, a: calls.append(a),
            )
        self.assertEqual(state["attempts"], 1)
        self.assertEqual(calls, [])

    async def test_backoff_strategies_drive_the_sleep_durations(self):
        expected = {
            "fixed": [0.5, 0.5, 0.5],
            "linear": [0.5, 1.0, 1.5],
            "exponential": [0.5, 1.0, 2.0],
        }
        for backoff in ("fixed", "linear", "exponential"):
            delays = expected[backoff]
            fn, _state = _failing(3)
            slept = []

            async def fake_sleep(seconds, slept=slept):
                slept.append(seconds)

            with patch("src.error.retry.asyncio.sleep", fake_sleep):
                result = await retry(fn, retries=3, delay=0.5, backoff=backoff)
            self.assertEqual(result, "ok")
            self.assertEqual(slept, delays)

    async def test_jitter_scales_the_sleep_duration(self):
        fn, _state = _failing(1)
        slept = []

        async def fake_sleep(seconds):
            slept.append(seconds)

        with (
            patch("src.error.retry.asyncio.sleep", fake_sleep),
            patch("src.error.retry.random_module.random", return_value=0.5),
        ):
            await retry(fn, retries=1, delay=4.0, jitter=True)
        self.assertEqual(slept, [2.0])

    async def test_large_retry_budget_does_not_recurse(self):
        fn, state = _failing(3000)
        result = await retry(fn, retries=3000, delay=0)
        self.assertEqual(result, "ok")
        self.assertEqual(state["attempts"], 3001)

    async def test_aborted_signal_raises_before_the_first_attempt(self):
        fn, state = _failing(0)
        signal = AbortSignal()
        signal.abort()

        with self.assertRaises(Exception) as context:
            await retry(fn, delay=0, signal=signal)
        self.assertEqual(str(context.exception), "Aborted")
        self.assertEqual(state["attempts"], 0)

    async def test_aborted_signal_raises_its_exception_reason(self):
        fn, _state = _failing(0)
        signal = AbortSignal()
        reason = RuntimeError("stop")
        signal.abort(reason)

        with self.assertRaises(RuntimeError) as context:
            await retry(fn, delay=0, signal=signal)
        self.assertIs(context.exception, reason)

    async def test_aborted_signal_wraps_a_non_exception_reason(self):
        fn, _state = _failing(0)
        signal = AbortSignal()
        signal.abort("cancelled by caller")

        with self.assertRaises(Exception) as context:
            await retry(fn, delay=0, signal=signal)
        self.assertEqual(str(context.exception), "cancelled by caller")

    async def test_abort_between_attempts_stops_retrying(self):
        fn, state = _failing(10)
        signal = AbortSignal()

        def on_retry(_error, _attempt):
            signal.abort()

        with self.assertRaises(Exception) as context:
            await retry(fn, retries=5, delay=0, on_retry=on_retry, signal=signal)
        self.assertEqual(str(context.exception), "Aborted")
        self.assertEqual(state["attempts"], 1)

    async def test_unaborted_signal_does_not_interfere(self):
        fn, state = _failing(1)
        result = await retry(fn, retries=2, delay=0, signal=AbortSignal())
        self.assertEqual(result, "ok")
        self.assertEqual(state["attempts"], 2)


if __name__ == "__main__":
    unittest.main()
