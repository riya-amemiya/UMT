import asyncio
from collections.abc import Awaitable, Callable
from typing import Any, Generic, TypeVar

R = TypeVar("R")


class DebouncedAsyncFunction(Generic[R]):
    """A debounced async callable.

    Subsequent calls within *wait* seconds reset the timer and share a single
    resolution; the latest arguments win. Mirrors the TypeScript
    ``debounceAsync`` helper. The TS version uses milliseconds; this Python
    port uses seconds.

    Attributes:
        cancel: Reject every pending caller with a cancellation error and
            clear the pending timer.

    Example:
        >>> import asyncio
        >>> async def main() -> list[int]:
        ...     async def double(n: int) -> int:
        ...         return n * 2
        ...     debounced = debounce_async(double, 0.01)
        ...     return list(
        ...         await asyncio.gather(
        ...             debounced(1), debounced(2), debounced(3)
        ...         )
        ...     )
        >>> asyncio.run(main())
        [6, 6, 6]
    """

    def __init__(
        self,
        function_: Callable[..., Awaitable[R]],
        wait: float,
    ) -> None:
        self._function = function_
        self._wait = wait
        self._timer: asyncio.TimerHandle | None = None
        self._pending: list[asyncio.Future[R]] = []

    def _flush_pending(self) -> "list[asyncio.Future[R]]":
        pending = self._pending
        self._pending = []
        return pending

    def _on_timer(self, args: tuple[Any, ...], kwargs: dict[str, Any]) -> None:
        self._timer = None
        pending = self._flush_pending()
        task = asyncio.ensure_future(self._function(*args, **kwargs))

        def _done(completed: "asyncio.Future[R]") -> None:
            error = completed.exception()
            if error is not None:
                for future in pending:
                    if not future.done():
                        future.set_exception(error)
            else:
                value = completed.result()
                for future in pending:
                    if not future.done():
                        future.set_result(value)

        task.add_done_callback(_done)

    def __call__(self, *args: Any, **kwargs: Any) -> "asyncio.Future[R]":  # noqa: ANN401
        if self._timer is not None:
            self._timer.cancel()
        loop = asyncio.get_event_loop()
        future: asyncio.Future[R] = loop.create_future()
        self._pending.append(future)
        self._timer = loop.call_later(self._wait, self._on_timer, args, kwargs)
        return future

    def cancel(self) -> None:
        """Reject all pending callers and clear the pending timer."""
        if self._timer is not None:
            self._timer.cancel()
            self._timer = None
        for future in self._flush_pending():
            if not future.done():
                future.set_exception(Exception("debounceAsync cancelled"))


def debounce_async(
    function_: Callable[..., Awaitable[R]],
    wait: float,
) -> DebouncedAsyncFunction[R]:
    """Create a debounced async function.

    Subsequent calls within *wait* seconds reset the timer and share a single
    resolution; the latest arguments win. Mirrors the TypeScript
    ``debounceAsync`` helper, using seconds instead of milliseconds.

    Args:
        function_: The async function to debounce.
        wait: Debounce window in seconds.

    Returns:
        A :class:`DebouncedAsyncFunction` whose call returns an awaitable and
        which exposes a ``cancel()`` method.

    Example:
        >>> import asyncio
        >>> async def main() -> int:
        ...     async def echo(n: int) -> int:
        ...         return n
        ...     debounced = debounce_async(echo, 0.01)
        ...     return await debounced(7)
        >>> asyncio.run(main())
        7
    """
    return DebouncedAsyncFunction(function_, wait)
