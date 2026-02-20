from collections.abc import Callable
from typing import ParamSpec

P = ParamSpec("P")


def every(*predicates: Callable[P, bool]) -> Callable[P, bool]:
    """
    Creates a predicate that returns true only when all given
    predicates return true, using short-circuit evaluation.

    Args:
        *predicates: The predicates to combine

    Returns:
        A combined predicate

    Example:
        >>> is_positive_even = every(
        ...   lambda n: n > 0,
        ...   lambda n: n % 2 == 0,
        ... )
        >>> is_positive_even(4)
        True
        >>> is_positive_even(-2)
        False
    """

    def combined(*args: P.args, **kwargs: P.kwargs) -> bool:
        return all(predicate(*args, **kwargs) for predicate in predicates)

    return combined
