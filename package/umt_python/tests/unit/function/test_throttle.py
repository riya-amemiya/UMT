import time
import unittest

from src.function import throttle


class TestThrottle(unittest.TestCase):
    def test_invokes_immediately_on_first_call(self):
        calls: list[int] = []
        throttled = throttle(lambda: calls.append(1), 0.1)

        throttled()
        self.assertEqual(len(calls), 1)
        throttled.cancel()

    def test_throttles_subsequent_calls_within_wait_period(self):
        calls: list[int] = []
        throttled = throttle(lambda: calls.append(1), 0.1)

        throttled()
        throttled()
        throttled()

        # Only the first call should have been executed immediately.
        self.assertEqual(len(calls), 1)
        throttled.cancel()

    def test_trailing_call_fires_after_wait_period(self):
        calls: list[int] = []
        throttled = throttle(lambda n: calls.append(n), 0.1)

        throttled(1)
        throttled(2)

        time.sleep(0.15)
        self.assertEqual(len(calls), 2)
        self.assertEqual(calls[-1], 2)
        throttled.cancel()

    def test_passes_latest_arguments_to_trailing_call(self):
        calls: list[str] = []
        throttled = throttle(lambda x: calls.append(x), 0.1)

        throttled("a")
        throttled("b")
        throttled("c")

        time.sleep(0.15)
        self.assertEqual(calls[-1], "c")
        throttled.cancel()

    def test_allows_invocation_after_wait_period(self):
        calls: list[int] = []
        throttled = throttle(lambda: calls.append(1), 0.1)

        throttled()
        time.sleep(0.15)

        throttled()
        self.assertEqual(len(calls), 2)
        throttled.cancel()

    def test_cancel_prevents_pending_invocation(self):
        calls: list[int] = []
        throttled = throttle(lambda: calls.append(1), 0.1)

        throttled()
        throttled()
        throttled.cancel()

        time.sleep(0.15)
        # Only the first immediate call should be present.
        self.assertEqual(len(calls), 1)

    def test_cancel_when_no_timer_pending(self):
        calls: list[int] = []
        throttled = throttle(lambda: calls.append(1), 0.1)

        # Should not raise.
        throttled.cancel()
        self.assertEqual(len(calls), 0)

    def test_kwargs_are_forwarded(self):
        calls: list[dict[str, int]] = []
        throttled = throttle(lambda **kw: calls.append(kw), 0.1)

        throttled(x=1)
        self.assertEqual(calls, [{"x": 1}])
        throttled.cancel()

    def test_multiple_wait_cycles(self):
        calls: list[int] = []
        throttled = throttle(lambda n: calls.append(n), 0.05)

        throttled(1)
        time.sleep(0.08)
        throttled(2)
        time.sleep(0.08)
        throttled(3)
        time.sleep(0.08)

        self.assertEqual(calls, [1, 2, 3])
        throttled.cancel()


if __name__ == "__main__":
    unittest.main()
