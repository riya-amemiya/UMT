from collections.abc import Callable, Iterable
from typing import TypeVar

T = TypeVar("T")


def ordered_unique(
    values: Iterable[T],
    key: Callable[[T], object] | None = None,
    on_duplicate: Callable[[T], object] | None = None,
) -> list[T]:
    """
    Collect the first value of each distinct key, in encounter order.

    Keys are compared with set membership (hash and equality). The first value of
    each key is returned unchanged. Unhashable keys raise ``TypeError``.

    Args:
        values: Values to scan.
        key: Maps a value to its identity key. Defaults to the value itself.
        on_duplicate: Called with every value whose key was already seen.

    Returns:
        The first-seen value of each key.

    Example:
        >>> ordered_unique([3, 1, 3, 2, 1])
        [3, 1, 2]
        >>> ordered_unique(["apple", "avocado", "banana"], key=lambda s: s[0])
        ['apple', 'banana']
    """
    seen: set[object] = set()
    result: list[T] = []
    for value in values:
        identity = value if key is None else key(value)
        if identity in seen:
            if on_duplicate is not None:
                on_duplicate(value)
            continue
        seen.add(identity)
        result.append(value)
    return result
