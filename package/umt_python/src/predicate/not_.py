from collections.abc import Callable
from typing import ParamSpec

P = ParamSpec("P")


def not_(predicate: Callable[P, bool]) -> Callable[P, bool]:
    """
    Creates a predicate that negates the given predicate

    Args:
        predicate: The predicate to negate

    Returns:
        A new predicate that returns the opposite

    Example:
        >>> is_even = lambda n: n % 2 == 0
        >>> is_odd = not_(is_even)
        >>> is_odd(3)
        True
    """

    def negated(*args: P.args, **kwargs: P.kwargs) -> bool:
        return not predicate(*args, **kwargs)

    return negated
