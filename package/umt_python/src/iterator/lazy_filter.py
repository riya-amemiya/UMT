from collections.abc import Callable, Generator, Iterable
from typing import TypeVar

T = TypeVar("T")


def lazy_filter(
    iterable: Iterable[T], predicate: Callable[[T, int], bool]
) -> Generator[T, None, None]:
    """
    Lazily filters values from an iterable using a generator.

    :param iterable: The source iterable.
    :param predicate: The filter predicate.
    :return: A generator yielding filtered values.
    """
    for index, value in enumerate(iterable):
        if predicate(value, index):
            yield value
