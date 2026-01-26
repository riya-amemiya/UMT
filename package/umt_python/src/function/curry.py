import functools
from collections.abc import Callable
from typing import Any


def curry(fn: Callable[..., Any]) -> Callable[..., Any]:
    """
    Curries a function.

    Args:
        fn: The function to curry

    Returns:
        The curried function

    Example:
        >>> def add(a, b, c):
        ...     return a + b + c
        >>> curried_add = curry(add)
        >>> curried_add(1)(2)(3)
        6
        >>> curried_add(1, 2)(3)
        6
        >>> curried_add(1, 2, 3)
        6
    """

    @functools.wraps(fn)
    def curried(*args: Any) -> Any:  # noqa: ANN401
        if len(args) >= fn.__code__.co_argcount:
            return fn(*args)
        return lambda *more_args: curried(*args, *more_args)

    return curried
