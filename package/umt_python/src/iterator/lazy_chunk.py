from collections.abc import Generator, Iterable
from typing import TypeVar

T = TypeVar("T")


def lazy_chunk(iterable: Iterable[T], size: int) -> Generator[list[T], None, None]:
    """
    Lazily groups values from an iterable into lists of the given size.

    The last chunk holds the remaining values and may be shorter than size.

    :param iterable: The source iterable.
    :param size: The number of values in each chunk.
    :return: A generator yielding chunks.

    Example:
        >>> list(lazy_chunk([1, 2, 3, 4, 5], 2))
        [[1, 2], [3, 4], [5]]
    """
    buffer: list[T] = []
    for value in iterable:
        buffer.append(value)
        if len(buffer) == size:
            yield buffer
            buffer = []
    if buffer:
        yield buffer
