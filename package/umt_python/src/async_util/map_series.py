from collections.abc import Awaitable, Callable
from typing import TypeVar

T = TypeVar("T")
U = TypeVar("U")


async def map_series(
    items: list[T],
    function_: Callable[[T, int], Awaitable[U]],
) -> list[U]:
    """Map over items sequentially, awaiting each call before the next.

    Mirrors the TypeScript ``mapSeries`` helper. Tasks never overlap, results
    preserve the order of *items*, and the first error propagates without
    invoking the remaining items.

    Args:
        items: The items to process.
        function_: The async mapper applied to each ``(item, index)``.

    Returns:
        Results in the same order as the input items.

    Example:
        >>> import asyncio
        >>> async def double(n: int, _index: int) -> int:
        ...     return n * 2
        >>> asyncio.run(map_series([1, 2, 3], double))
        [2, 4, 6]
    """
    return [await function_(items[index], index) for index in range(len(items))]
