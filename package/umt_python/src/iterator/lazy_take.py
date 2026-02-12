from collections.abc import Generator, Iterable
from itertools import islice
from typing import TypeVar

T = TypeVar("T")


def lazy_take(iterable: Iterable[T], n: int) -> Generator[T, None, None]:
    """
    Lazily takes the first n values from an iterable.

    :param iterable: The source iterable.
    :param n: The number of values to take.
    :return: A generator yielding at most n values.
    """
    yield from islice(iterable, n)
