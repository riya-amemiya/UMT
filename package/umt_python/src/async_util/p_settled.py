import asyncio
from collections.abc import Awaitable, Callable, Iterable
from dataclasses import dataclass, field
from typing import Any, Generic, TypeVar, cast

T = TypeVar("T")

Task = Awaitable[T] | Callable[[], Awaitable[T]]


@dataclass
class SettledResult(Generic[T]):
    """The settled outcome of a single task.

    ``status`` is ``"fulfilled"`` with a ``value`` or ``"rejected"`` with a
    ``reason``. Mirrors the TypeScript ``SettledResult`` union.

    Attributes:
        status: Either ``"fulfilled"`` or ``"rejected"``.
        value: The resolved value when fulfilled, otherwise ``None``.
        reason: The raised error when rejected, otherwise ``None``.
    """

    status: str
    value: Any = field(default=None)
    reason: Any = field(default=None)


async def _run_task(task: Task[T]) -> SettledResult[T]:
    try:
        awaitable = task() if callable(task) else task
        value = await awaitable
    except Exception as error:
        return SettledResult("rejected", reason=error)
    return SettledResult("fulfilled", value=value)


async def p_settled(
    tasks: Iterable[Task[T]],
    limit: int | None = None,
) -> list[SettledResult[T]]:
    """Await all tasks and return their settled results, honoring an optional
    concurrency limit.

    Mirrors the TypeScript ``pSettled`` helper. Accepts awaitables or thunks
    (zero-argument callables returning an awaitable). Results preserve input
    order. A *limit* of ``0`` (or ``None``) is treated as unlimited.

    Args:
        tasks: Awaitables or thunks to settle.
        limit: Maximum concurrent in-flight tasks; unlimited when falsy.

    Returns:
        Settled results in input order.

    Example:
        >>> import asyncio
        >>> async def main() -> list[str]:
        ...     async def ok() -> int:
        ...         return 1
        ...     async def bad() -> int:
        ...         raise ValueError("nope")
        ...     out = await p_settled([ok(), bad()])
        ...     return [r.status for r in out]
        >>> asyncio.run(main())
        ['fulfilled', 'rejected']
    """
    items: list[Task[T]] = list(tasks)
    results: list[SettledResult[T]] = cast(
        "list[SettledResult[T]]", [None] * len(items)
    )
    if not items:
        return results

    effective_limit = limit if limit and limit > 0 else len(items)
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
            results[current_index] = await _run_task(items[current_index])

    initial_batch = min(effective_limit, len(items))
    await asyncio.gather(*(worker() for _ in range(initial_batch)))
    return results
