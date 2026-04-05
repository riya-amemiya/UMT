import threading
import time
from collections.abc import Callable
from typing import Any, ParamSpec, TypeVar

P = ParamSpec("P")
R = TypeVar("R")


class ThrottledFunction:
    """A throttled callable that invokes the wrapped function at most once
    per *wait* seconds.

    The first call is executed immediately.  Subsequent calls within the
    wait window are coalesced: only the **latest** arguments are kept and
    a single trailing invocation fires once the window expires.

    Attributes:
        cancel: Cancel any pending trailing invocation and reset internal
            state.

    Example:
        >>> import time
        >>> calls: list[int] = []
        >>> def track(n: int) -> None:
        ...     calls.append(n)
        >>> throttled = throttle(track, 0.1)
        >>> throttled(1)
        >>> throttled(2)
        >>> throttled(3)
        >>> time.sleep(0.15)
        >>> calls
        [1, 3]
    """

    def __init__(
        self,
        func: Callable[..., Any],
        wait: float,
    ) -> None:
        self._func = func
        self._wait = wait
        self._timer: threading.Timer | None = None
        self._last_call_time: float = 0.0
        self._last_args: tuple[Any, ...] | None = None
        self._last_kwargs: dict[str, Any] | None = None
        self._lock = threading.Lock()

    def _invoke(self) -> None:
        with self._lock:
            self._last_call_time = time.monotonic()
            args = self._last_args
            kwargs = self._last_kwargs
            self._last_args = None
            self._last_kwargs = None
            self._timer = None
        if args is not None:
            self._func(*args, **kwargs or {})

    def __call__(self, *args: Any, **kwargs: Any) -> None:  # noqa: ANN401
        with self._lock:
            self._last_args = args
            self._last_kwargs = kwargs
            elapsed = time.monotonic() - self._last_call_time
            remaining = self._wait - elapsed

            if remaining <= 0:
                # Enough time has passed; invoke immediately.
                if self._timer is not None:
                    self._timer.cancel()
                    self._timer = None
                self._last_call_time = time.monotonic()
                captured_args = args
                captured_kwargs = kwargs
                self._last_args = None
                self._last_kwargs = None
            else:
                # Schedule a trailing call if not already scheduled.
                if self._timer is None:
                    self._timer = threading.Timer(remaining, self._invoke)
                    self._timer.daemon = True
                    self._timer.start()
                return

        # Invoke outside the lock to avoid deadlocks.
        self._func(*captured_args, **captured_kwargs)

    def cancel(self) -> None:
        """Cancel any pending trailing invocation and reset state."""
        with self._lock:
            if self._timer is not None:
                self._timer.cancel()
                self._timer = None
            self._last_args = None
            self._last_kwargs = None
            self._last_call_time = 0.0


def throttle(
    func: Callable[P, Any],
    wait: float,
) -> ThrottledFunction:
    """Create a throttled version of *func* that invokes at most once per
    *wait* seconds.

    The first call is executed immediately.  If additional calls arrive
    before the wait period expires, the **latest** arguments are captured
    and a single trailing invocation occurs once the period elapses.

    Args:
        func: The function to throttle.
        wait: Minimum interval between invocations, in seconds.

    Returns:
        A :class:`ThrottledFunction` wrapper with a ``cancel()`` method.

    Example:
        >>> import time
        >>> results: list[str] = []
        >>> throttled = throttle(lambda x: results.append(x), 0.1)
        >>> throttled("a")
        >>> throttled("b")
        >>> time.sleep(0.15)
        >>> results
        ['a', 'b']
    """
    return ThrottledFunction(func, wait)
