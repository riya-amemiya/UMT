from collections.abc import Generator, Iterable

_DONE = object()


def lazy_zip(*iterables: Iterable[object]) -> Generator[list[object], None, None]:
    """
    Lazily combines values from several iterables at corresponding positions.

    Stops as soon as any iterable is exhausted and then closes every source
    iterator that has a ``close`` method.

    :param iterables: The source iterables.
    :return: A generator yielding one list per position.

    Example:
        >>> list(lazy_zip([1, 2, 3], ["a", "b"]))
        [[1, 'a'], [2, 'b']]
    """
    if len(iterables) == 0:
        return
    iterators = [iter(iterable) for iterable in iterables]
    try:
        while True:
            row: list[object] = []
            for iterator in iterators:
                value = next(iterator, _DONE)
                if value is _DONE:
                    return
                row.append(value)
            yield row
    finally:
        for iterator in iterators:
            close = getattr(iterator, "close", None)
            if close is not None:
                close()
