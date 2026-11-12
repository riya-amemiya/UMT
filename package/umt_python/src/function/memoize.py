import time
from collections import OrderedDict
from collections.abc import Callable
from typing import Any


class MemoizedFunction:
    """
    A memoized function with an exposed cache.

    The cache is an OrderedDict that maps keys to computed results.
    When called, the memoized function checks the cache before computing.
    When a ttl is set, an entry older than ttl milliseconds is recomputed on
    its next access.
    """

    def __init__(
        self,
        fn: Callable[..., Any],
        max_size: int | None = None,
        resolver: Callable[..., Any] | None = None,
        ttl: int | float | None = None,
    ) -> None:
        self._fn = fn
        self._max_size = max_size
        self._resolver = resolver
        self._ttl = ttl
        self._expirations: dict[object, float] = {}
        self.cache: OrderedDict[Any, Any] = OrderedDict()

    def _is_expired(self, key: object) -> bool:
        return (
            self._ttl is not None
            and self._expirations.get(key, 0.0) <= time.monotonic() * 1000
        )

    def __call__(self, *args: Any) -> Any:  # noqa: ANN401
        if self._resolver is not None:
            key = self._resolver(*args)
        else:
            key = args[0] if args else None

        if key in self.cache and not self._is_expired(key):
            return self.cache[key]

        self.cache.pop(key, None)
        self._expirations.pop(key, None)
        result = self._fn(*args)

        if (
            self._max_size is not None
            and self.cache
            and len(self.cache) >= self._max_size
        ):
            oldest_key, _ = self.cache.popitem(last=False)
            self._expirations.pop(oldest_key, None)

        self.cache[key] = result
        if self._ttl is not None:
            self._expirations[key] = time.monotonic() * 1000 + self._ttl
        return result


def memoize(
    fn: Callable[..., Any],
    *,
    max_size: int | None = None,
    resolver: Callable[..., Any] | None = None,
    ttl: int | float | None = None,
) -> MemoizedFunction:
    """
    Creates a memoized version of the provided function.

    Results are cached and keyed by the first argument by default,
    or by a custom resolver function if provided.

    Args:
        fn: The function to memoize.
        max_size: Maximum cache entries before evicting the oldest.
        resolver: Custom key resolver function.
        ttl: Entry lifetime in milliseconds; expired entries are recomputed
            on access.

    Returns:
        A MemoizedFunction with an exposed cache attribute.

    Example:
        >>> memoized = memoize(lambda n: n * 2)
        >>> memoized(5)
        10
        >>> memoized(5)
        10
        >>> len(memoized.cache)
        1
        >>> timed = memoize(lambda n: n * 2, ttl=1000)
        >>> timed(5)
        10
    """
    return MemoizedFunction(fn, max_size=max_size, resolver=resolver, ttl=ttl)
