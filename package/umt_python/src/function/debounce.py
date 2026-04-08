import threading
import time
from collections.abc import Callable
from typing import Any, ParamSpec

P = ParamSpec("P")


class DebouncedFunction:
    """A debounced callable that delays invoking the wrapped function until
    after *wait* seconds have elapsed since the last call.

    Supports *leading* and *trailing* edge invocation via the constructor
    parameters.

    Attributes:
        cancel: Cancel any pending debounced invocation and reset internal
            state.

    Example:
        >>> import time
        >>> calls: list[int] = []
        >>> def track(n: int) -> None:
        ...     calls.append(n)
        >>> debounced = debounce(track, 0.1)
        >>> debounced(1)
        >>> debounced(2)
        >>> debounced(3)
        >>> time.sleep(0.15)
        >>> calls
        [3]
    """

    def __init__(
        self,
        func: Callable[..., Any],
        wait: float,
        *,
        leading: bool = False,
        trailing: bool = True,
    ) -> None:
        self._func = func
        self._wait = wait
        self._leading = leading
        self._trailing = trailing
        self._timer: threading.Timer | None = None
        self._last_call_time: float = 0.0
        self._last_args: tuple[Any, ...] | None = None
        self._last_kwargs: dict[str, Any] | None = None
        self._lock = threading.Lock()

    def _schedule_check(self) -> None:
        """Schedule a timer that fires after the remaining wait time.

        When the timer fires, if enough time has elapsed since the last call
        the trailing invocation is performed.  Otherwise, the timer is
        rescheduled for the remaining duration.
        """
        with self._lock:
            elapsed = time.monotonic() - self._last_call_time
            remaining = self._wait - elapsed

        if remaining <= 0:
            self._fire_trailing()
        else:
            with self._lock:
                self._timer = threading.Timer(remaining, self._on_timer)
                self._timer.daemon = True
                self._timer.start()

    def _on_timer(self) -> None:
        """Callback executed when the scheduled timer fires."""
        with self._lock:
            elapsed = time.monotonic() - self._last_call_time
            remaining = self._wait - elapsed

        if remaining <= 0:
            self._fire_trailing()
        else:
            with self._lock:
                self._timer = threading.Timer(remaining, self._on_timer)
                self._timer.daemon = True
                self._timer.start()

    def _fire_trailing(self) -> None:
        """Invoke the function on the trailing edge if applicable."""
        with self._lock:
            self._timer = None
            if self._trailing and self._last_args is not None:
                args = self._last_args
                kwargs = self._last_kwargs
                self._last_args = None
                self._last_kwargs = None
            else:
                self._last_args = None
                self._last_kwargs = None
                return

        self._func(*args, **(kwargs or {}))

    def __call__(self, *args: Any, **kwargs: Any) -> None:  # noqa: ANN401
        with self._lock:
            self._last_args = args
            self._last_kwargs = kwargs
            self._last_call_time = time.monotonic()
            is_first_call = self._timer is None

            if self._leading and is_first_call:
                captured_args = args
                captured_kwargs = kwargs
                self._last_args = None
                self._last_kwargs = None
            else:
                captured_args = None
                captured_kwargs = None

            # Cancel any existing timer so we can reschedule.
            if self._timer is not None:
                self._timer.cancel()
                self._timer = None

        # Leading-edge invocation (outside lock).
        if captured_args is not None:
            self._func(*captured_args, **(captured_kwargs or {}))

        # Schedule a new timer for the trailing edge.
        with self._lock:
            self._timer = threading.Timer(self._wait, self._on_timer)
            self._timer.daemon = True
            self._timer.start()

    def cancel(self) -> None:
        """Cancel any pending debounced invocation and reset state."""
        with self._lock:
            if self._timer is not None:
                self._timer.cancel()
                self._timer = None
            self._last_args = None
            self._last_kwargs = None
            self._last_call_time = 0.0


def debounce(
    func: Callable[P, Any],
    wait: float,
    *,
    leading: bool = False,
    trailing: bool = True,
) -> DebouncedFunction:
    """Create a debounced version of *func* that delays invocation until
    after *wait* seconds have elapsed since the last call.

    Args:
        func: The function to debounce.
        wait: The delay in seconds before the function is invoked.
        leading: If ``True``, invoke on the leading edge of the wait period.
        trailing: If ``True`` (default), invoke on the trailing edge.

    Returns:
        A :class:`DebouncedFunction` wrapper with a ``cancel()`` method.

    Example:
        >>> import time
        >>> results: list[str] = []
        >>> debounced = debounce(lambda x: results.append(x), 0.1)
        >>> debounced("a")
        >>> debounced("b")
        >>> time.sleep(0.15)
        >>> results
        ['b']
    """
    return DebouncedFunction(func, wait, leading=leading, trailing=trailing)
