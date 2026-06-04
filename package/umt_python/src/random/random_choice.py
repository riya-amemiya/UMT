import math
import random as random_module
from collections.abc import Sequence
from typing import TypeVar

T = TypeVar("T")


def random_choice(items: Sequence[T]) -> T:
    """
    Returns a uniformly random element from the input sequence.

    Args:
        items: Source sequence (must be non-empty).

    Returns:
        A randomly selected element.

    Example:
        >>> random_choice([42])
        42
        >>> random_choice(["a", "b", "c"]) in ("a", "b", "c")
        True
    """
    index = math.floor(random_module.random() * len(items))
    return items[index]
