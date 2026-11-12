import asyncio
from collections import OrderedDict
from collections.abc import Awaitable, Callable
from typing import Generic, TypeVar

R = TypeVar("R")


class MemoizedAsyncFunction(Generic[R]):
    """
    A memoized async function with an exposed cache of resolved values.

    Concurrent calls that share a key await a single in-flight task.
    Resolved values are stored in ``cache``; raised errors are not cached.

    Example:
        >>> import asyncio
        >>> async def main() -> tuple[list[int], int]:
        ...     async def double(n: int) -> int:
        ...         return n * 2
        ...     memoized = memoize_async(double)
        ...     values = list(await asyncio.gather(memoized(2), memoized(2)))
        ...     return values, len(memoized.cache)
        >>> asyncio.run(main())
        ([4, 4], 1)
    """

    def __init__(
        self,
        fn: Callable[..., Awaitable[R]],
        max_size: int | None = None,
        resolver: Callable[..., object] | None = None,
    ) -> None:
        self._fn = fn
        self._max_size = max_size
        self._resolver = resolver
        self._pending: dict[object, asyncio.Task[R]] = {}
        self.cache: OrderedDict[object, R] = OrderedDict()

    async def _run(self, key: object, args: tuple[object, ...]) -> R:
        try:
            value = await self._fn(*args)
        finally:
            self._pending.pop(key, None)

        if (
            self._max_size is not None
            and self.cache
            and len(self.cache) >= self._max_size
        ):
            self.cache.popitem(last=False)

        self.cache[key] = value
        return value

    async def __call__(self, *args: object) -> R:
        if self._resolver is not None:
            key = self._resolver(*args)
        else:
            key = args[0] if args else None

        if key in self.cache:
            return self.cache[key]

        task = self._pending.get(key)
        if task is None:
            task = asyncio.ensure_future(self._run(key, args))
            self._pending[key] = task
        return await task


def memoize_async(
    fn: Callable[..., Awaitable[R]],
    *,
    max_size: int | None = None,
    resolver: Callable[..., object] | None = None,
) -> MemoizedAsyncFunction[R]:
    """
    Creates a memoized version of an async function.

    Concurrent calls that share a key reuse one in-flight task, and resolved
    values are cached keyed by the first argument by default, or by a custom
    resolver function if provided. Raised errors are not cached, so the next
    call with the same key invokes the function again.

    Args:
        fn: The async function to memoize.
        max_size: Maximum cache entries before evicting the oldest.
        resolver: Custom key resolver function.

    Returns:
        A MemoizedAsyncFunction with an exposed cache attribute.

    Example:
        >>> import asyncio
        >>> async def main() -> int:
        ...     async def square(n: int) -> int:
        ...         return n * n
        ...     memoized = memoize_async(square)
        ...     await memoized(3)
        ...     return await memoized(3)
        >>> asyncio.run(main())
        9
    """
    return MemoizedAsyncFunction(fn, max_size=max_size, resolver=resolver)
