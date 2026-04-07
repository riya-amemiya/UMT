import time
import unittest

from src.function import debounce


class TestDebounce(unittest.TestCase):
    def test_delays_invocation_until_after_wait_period(self):
        calls: list[int] = []
        debounced = debounce(lambda: calls.append(1), 0.1)

        debounced()
        self.assertEqual(len(calls), 0)

        time.sleep(0.15)
        self.assertEqual(len(calls), 1)
        debounced.cancel()

    def test_resets_timer_on_subsequent_calls(self):
        calls: list[int] = []
        debounced = debounce(lambda: calls.append(1), 0.1)

        debounced()
        time.sleep(0.05)
        debounced()
        time.sleep(0.05)
        # Should not have fired yet because the second call reset the timer.
        self.assertEqual(len(calls), 0)

        time.sleep(0.1)
        self.assertEqual(len(calls), 1)
        debounced.cancel()

    def test_passes_latest_arguments(self):
        calls: list[int] = []
        debounced = debounce(lambda n: calls.append(n), 0.1)

        debounced(1)
        debounced(2)
        debounced(3)

        time.sleep(0.15)
        self.assertEqual(calls, [3])
        debounced.cancel()

    def test_leading_edge_invocation(self):
        calls: list[int] = []
        debounced = debounce(lambda: calls.append(1), 0.1, leading=True, trailing=False)

        debounced()
        self.assertEqual(len(calls), 1)

        debounced()
        self.assertEqual(len(calls), 1)

        time.sleep(0.15)
        # trailing=False means no additional call.
        self.assertEqual(len(calls), 1)
        debounced.cancel()

    def test_leading_and_trailing_edges(self):
        calls: list[int] = []
        debounced = debounce(
            lambda n: calls.append(n), 0.1, leading=True, trailing=True
        )

        debounced(1)
        self.assertEqual(len(calls), 1)
        self.assertEqual(calls[0], 1)

        debounced(2)
        time.sleep(0.15)
        self.assertEqual(len(calls), 2)
        self.assertEqual(calls[-1], 2)
        debounced.cancel()

    def test_cancel_prevents_pending_invocation(self):
        calls: list[int] = []
        debounced = debounce(lambda: calls.append(1), 0.1)

        debounced()
        debounced.cancel()

        time.sleep(0.15)
        self.assertEqual(len(calls), 0)

    def test_cancel_when_no_timer_pending(self):
        calls: list[int] = []
        debounced = debounce(lambda: calls.append(1), 0.1)

        # Should not raise.
        debounced.cancel()
        time.sleep(0.15)
        self.assertEqual(len(calls), 0)

    def test_kwargs_are_forwarded(self):
        calls: list[dict[str, int]] = []
        debounced = debounce(lambda **kw: calls.append(kw), 0.1)

        debounced(x=1)
        time.sleep(0.15)
        self.assertEqual(calls, [{"x": 1}])
        debounced.cancel()

    def test_multiple_wait_cycles(self):
        calls: list[int] = []
        debounced = debounce(lambda n: calls.append(n), 0.05)

        debounced(1)
        time.sleep(0.08)
        debounced(2)
        time.sleep(0.08)
        debounced(3)
        time.sleep(0.08)

        self.assertEqual(calls, [1, 2, 3])
        debounced.cancel()


if __name__ == "__main__":
    unittest.main()
