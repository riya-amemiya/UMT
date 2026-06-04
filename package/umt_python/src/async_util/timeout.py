import asyncio
from collections.abc import Awaitable
from typing import TypeVar

T = TypeVar("T")


async def timeout(awaitable: Awaitable[T], seconds: float) -> T:
    """Wrap an awaitable with a timeout, raising if it does not resolve in
    time.

    Mirrors the TypeScript ``timeout`` helper. The TS version rejects with
    ``Timed out after {ms}ms``; this Python port raises
    :class:`TimeoutError` with the same message shape, using seconds.

    Args:
        awaitable: The awaitable to wrap.
        seconds: The timeout in seconds.

    Returns:
        The result of *awaitable* when it completes before the timeout.

    Raises:
        TimeoutError: When *awaitable* does not complete within *seconds*.

    Example:
        >>> import asyncio
        >>> async def quick() -> str:
        ...     return "done"
        >>> asyncio.run(timeout(quick(), 1))
        'done'
    """
    task = asyncio.ensure_future(awaitable)
    try:
        return await asyncio.wait_for(asyncio.shield(task), seconds)
    except asyncio.TimeoutError:
        task.cancel()
        raise TimeoutError(f"Timed out after {seconds}s") from None
