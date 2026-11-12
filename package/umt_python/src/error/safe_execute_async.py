import inspect
from collections.abc import Awaitable, Callable
from typing import TypeVar, cast

from .safe_execute import Error, Result, Success

V = TypeVar("V")


async def safe_execute_async(
    callback: Callable[[], Awaitable[V] | V],
) -> Result[V, Exception]:
    """
    Safely executes an async callback and returns a Result type.
    Catches any errors and wraps them in a Result type.

    Args:
        callback: The async (or sync) function to execute safely

    Returns:
        A Result containing either the successful value or the caught error

    Example:
        >>> import asyncio
        >>> async def answer() -> int:
        ...     return 42
        >>> asyncio.run(safe_execute_async(answer)).value
        42
        >>> asyncio.run(safe_execute_async(lambda: 1 / 0)).type
        'error'
    """
    try:
        result = callback()
        value = await result if inspect.isawaitable(result) else result
        return Success(type="success", value=cast("V", value))
    except Exception as error:
        return Error(type="error", error=error)
