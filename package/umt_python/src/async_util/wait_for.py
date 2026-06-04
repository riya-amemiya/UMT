import asyncio
import inspect
import time
from collections.abc import Awaitable, Callable
from typing import Any, TypeVar, cast

T = TypeVar("T")


class AbortSignal:
    """A minimal abort signal mirroring the DOM ``AbortSignal`` used by the
    TypeScript ``waitFor`` helper.

    ``aborted`` reflects whether :meth:`abort` was called, and ``reason``
    carries the optional cancellation cause.
    """

    __slots__ = ("aborted", "reason")

    def __init__(self) -> None:
        self.aborted: bool = False
        self.reason: Any = None

    def abort(self, reason: Any = None) -> None:  # noqa: ANN401
        """Mark the signal aborted, recording an optional *reason*."""
        self.aborted = True
        self.reason = reason


async def wait_for(
    condition: Callable[[], T | Awaitable[T]],
    *,
    timeout: float = 5.0,
    interval: float = 0.1,
    signal: AbortSignal | None = None,
) -> T:
    """Repeatedly evaluate a condition until it returns a truthy value or the
    timeout elapses, resolving with the final truthy value.

    Mirrors the TypeScript ``waitFor`` helper. The TS version uses
    milliseconds (default ``timeout`` 5000, ``interval`` 100); this Python
    port uses seconds (default ``timeout`` 5.0, ``interval`` 0.1).

    Args:
        condition: A predicate returning a value or awaitable value.
        timeout: Maximum seconds to wait before giving up.
        interval: Seconds to wait between polls.
        signal: Optional abort signal that cancels the wait.

    Returns:
        The first truthy value produced by *condition*.

    Raises:
        Exception: When the timeout elapses or the signal aborts. The aborted
            reason is raised when present, otherwise a generic ``Aborted`` or
            timeout error.

    Example:
        >>> import asyncio
        >>> asyncio.run(wait_for(lambda: 42))
        42
    """
    start_time = time.monotonic()

    while True:
        if signal is not None and signal.aborted:
            if signal.reason is not None:
                if isinstance(signal.reason, BaseException):
                    raise signal.reason
                raise Exception(signal.reason)
            raise Exception("Aborted")

        result = condition()
        value = await result if inspect.isawaitable(result) else result
        if value:
            return cast("T", value)

        if time.monotonic() - start_time >= timeout:
            raise TimeoutError(f"waitFor timed out after {timeout}s")

        await asyncio.sleep(interval)
