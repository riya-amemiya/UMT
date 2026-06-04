import asyncio
import time
from collections.abc import Awaitable, Callable
from typing import Any, Generic, TypeVar

R = TypeVar("R")


class ThrottledAsyncFunction(Generic[R]):
    """A throttled async callable.

    Concurrent calls within the *wait* window coalesce so only one underlying
    invocation runs; all callers in the window receive the same in-flight
    awaitable. Mirrors the TypeScript ``throttleAsync`` helper. The TS version
    uses milliseconds; this Python port uses seconds.

    Attributes:
        cancel: Clear the in-flight reference and the lockout window.

    Example:
        >>> import asyncio
        >>> async def main() -> int:
        ...     calls = {"n": 0}
        ...     async def load(value: int) -> int:
        ...         calls["n"] += 1
        ...         await asyncio.sleep(0)
        ...         return value
        ...     throttled = throttle_async(load, 0)
        ...     a = throttled(1)
        ...     b = throttled(2)
        ...     await asyncio.gather(a, b)
        ...     return calls["n"]
        >>> asyncio.run(main())
        1
    """

    def __init__(
        self,
        function_: Callable[..., Awaitable[R]],
        wait: float,
    ) -> None:
        self._function = function_
        self._wait = wait
        self._inflight: asyncio.Future[R] | None = None
        self._locked_until: float = 0.0

    def __call__(self, *args: Any, **kwargs: Any) -> "asyncio.Future[R]":  # noqa: ANN401
        if self._inflight is not None:
            return self._inflight
        loop = asyncio.get_event_loop()
        if time.monotonic() < self._locked_until:
            future: asyncio.Future[R] = loop.create_future()
            future.set_exception(Exception("throttleAsync window not elapsed"))
            return future

        current: asyncio.Future[R] = asyncio.ensure_future(
            self._function(*args, **kwargs)
        )
        self._inflight = current

        def _cleanup(_completed: "asyncio.Future[R]") -> None:
            if self._inflight is current:
                self._inflight = None
                self._locked_until = time.monotonic() + self._wait

        current.add_done_callback(_cleanup)
        return current

    def cancel(self) -> None:
        """Clear the in-flight reference and reset the lockout window."""
        self._inflight = None
        self._locked_until = 0.0


def throttle_async(
    function_: Callable[..., Awaitable[R]],
    wait: float,
) -> ThrottledAsyncFunction[R]:
    """Create a throttled async function.

    Concurrent calls within the *wait* window coalesce so only one underlying
    invocation runs; all callers receive the same in-flight awaitable. Mirrors
    the TypeScript ``throttleAsync`` helper, using seconds instead of
    milliseconds. After completion, calls inside the lockout window reject with
    a window-not-elapsed error.

    Args:
        function_: The async function to throttle.
        wait: Window length in seconds.

    Returns:
        A :class:`ThrottledAsyncFunction` whose call returns an awaitable and
        which exposes a ``cancel()`` method.

    Example:
        >>> import asyncio
        >>> async def main() -> int:
        ...     async def echo(n: int) -> int:
        ...         return n
        ...     throttled = throttle_async(echo, 0.01)
        ...     return await throttled(5)
        >>> asyncio.run(main())
        5
    """
    return ThrottledAsyncFunction(function_, wait)
