from collections.abc import Callable, Generator, Iterable
from typing import TypeVar

T = TypeVar("T")
U = TypeVar("U")


def lazy_map(
    iterable: Iterable[T], function_: Callable[[T, int], U]
) -> Generator[U, None, None]:
    """
    Lazily maps values from an iterable using a generator.

    :param iterable: The source iterable.
    :param function_: The mapping function.
    :return: A generator yielding mapped values.
    """
    for index, value in enumerate(iterable):
        yield function_(value, index)
