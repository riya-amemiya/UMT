from collections.abc import Callable
from typing import ParamSpec

P = ParamSpec("P")


def some(*predicates: Callable[P, bool]) -> Callable[P, bool]:
    """
    Creates a predicate that returns true when at least one of
    the given predicates returns true, using short-circuit evaluation.

    Args:
        *predicates: The predicates to combine

    Returns:
        A combined predicate

    Example:
        >>> is_zero_or_negative = some(
        ...   lambda n: n == 0,
        ...   lambda n: n < 0,
        ... )
        >>> is_zero_or_negative(0)
        True
        >>> is_zero_or_negative(-1)
        True
        >>> is_zero_or_negative(1)
        False
    """

    def combined(*args: P.args, **kwargs: P.kwargs) -> bool:
        return any(predicate(*args, **kwargs) for predicate in predicates)

    return combined
