import asyncio
from collections.abc import Awaitable, Callable
from typing import TypeVar, cast

T = TypeVar("T")
U = TypeVar("U")


async def parallel(
    limit: int,
    items: list[T],
    function_: Callable[[T, int], Awaitable[U]],
) -> list[U]:
    """Execute async functions in parallel with a concurrency limit.

    Mirrors the TypeScript ``parallel`` helper. Results preserve the order of
    *items*. If any task raises, the first error propagates.

    Args:
        limit: Maximum number of concurrent executions.
        items: The items to process.
        function_: The async function applied to each ``(item, index)``.

    Returns:
        Results in the same order as the input items.

    Example:
        >>> import asyncio
        >>> async def double(n: int, _index: int) -> int:
        ...     return n * 2
        >>> asyncio.run(parallel(2, [1, 2, 3], double))
        [2, 4, 6]
    """
    results: list[U] = cast("list[U]", [None] * len(items))
    if not items:
        return results

    next_index = 0
    lock = asyncio.Lock()

    async def worker() -> None:
        nonlocal next_index
        while True:
            async with lock:
                if next_index >= len(items):
                    return
                current_index = next_index
                next_index += 1
            results[current_index] = await function_(
                items[current_index], current_index
            )

    initial_batch = min(limit, len(items))
    await asyncio.gather(*(worker() for _ in range(initial_batch)))
    return results
