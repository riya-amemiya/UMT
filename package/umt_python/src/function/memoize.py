from collections import OrderedDict
from collections.abc import Callable
from typing import Any


class MemoizedFunction:
    """
    A memoized function with an exposed cache.

    The cache is an OrderedDict that maps keys to computed results.
    When called, the memoized function checks the cache before computing.
    """

    def __init__(
        self,
        fn: Callable[..., Any],
        max_size: int | None = None,
        resolver: Callable[..., Any] | None = None,
    ) -> None:
        self._fn = fn
        self._max_size = max_size
        self._resolver = resolver
        self.cache: OrderedDict[Any, Any] = OrderedDict()

    def __call__(self, *args: Any) -> Any:  # noqa: ANN401
        key = self._resolver(*args) if self._resolver is not None else args[0]

        if key in self.cache:
            return self.cache[key]

        result = self._fn(*args)

        if self._max_size is not None and len(self.cache) >= self._max_size:
            self.cache.popitem(last=False)

        self.cache[key] = result
        return result


def memoize(
    fn: Callable[..., Any],
    *,
    max_size: int | None = None,
    resolver: Callable[..., Any] | None = None,
) -> MemoizedFunction:
    """
    Creates a memoized version of the provided function.

    Results are cached and keyed by the first argument by default,
    or by a custom resolver function if provided.

    Args:
        fn: The function to memoize.
        max_size: Maximum cache entries before evicting the oldest.
        resolver: Custom key resolver function.

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
    """
    return MemoizedFunction(fn, max_size=max_size, resolver=resolver)
