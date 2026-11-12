from collections import deque
from collections.abc import Iterator
from typing import Generic, TypeVar

T = TypeVar("T")


class Deque(Generic[T]):
    """
    A double-ended queue. Elements can be added and removed at both ends.

    Features:
        push_front(value): Add element to the front
        push_back(value): Add element to the back
        pop_front(): Remove and return the front element
        pop_back(): Remove and return the back element
        peek_front(): View the front element without removing
        peek_back(): View the back element without removing
        size: Get number of elements
        is_empty: Check if deque is empty
        clear(): Remove all elements
        iter(deque): Iterate from front to back

    Time Complexity:
        push_front / push_back: O(1)
        pop_front / pop_back: O(1)
        peek_front / peek_back: O(1)
        size / is_empty: O(1)

    Example:
        >>> d = Deque()
        >>> d.push_back(2)
        >>> d.push_back(3)
        >>> d.push_front(1)
        >>> list(d)
        [1, 2, 3]
        >>> d.pop_front()
        1
        >>> d.pop_back()
        3
        >>> d.size
        1
    """

    def __init__(self) -> None:
        """Creates a new empty Deque instance."""
        self._items: deque[T] = deque()

    @property
    def size(self) -> int:
        """Returns the number of elements in the deque."""
        return len(self._items)

    @property
    def is_empty(self) -> bool:
        """Checks if the deque is empty."""
        return len(self._items) == 0

    def push_front(self, value: T) -> None:
        """
        Adds an element to the front of the deque.

        Args:
            value: The value to add
        """
        self._items.appendleft(value)

    def push_back(self, value: T) -> None:
        """
        Adds an element to the back of the deque.

        Args:
            value: The value to add
        """
        self._items.append(value)

    def pop_front(self) -> T | None:
        """
        Removes and returns the front element.

        Returns:
            The front element, or None if the deque is empty
        """
        if len(self._items) == 0:
            return None
        return self._items.popleft()

    def pop_back(self) -> T | None:
        """
        Removes and returns the back element.

        Returns:
            The back element, or None if the deque is empty
        """
        if len(self._items) == 0:
            return None
        return self._items.pop()

    def peek_front(self) -> T | None:
        """
        Returns the front element without removing it.

        Returns:
            The front element, or None if the deque is empty
        """
        if len(self._items) == 0:
            return None
        return self._items[0]

    def peek_back(self) -> T | None:
        """
        Returns the back element without removing it.

        Returns:
            The back element, or None if the deque is empty
        """
        if len(self._items) == 0:
            return None
        return self._items[-1]

    def clear(self) -> None:
        """Removes all elements from the deque."""
        self._items.clear()

    def __iter__(self) -> Iterator[T]:
        """
        Iterates over the elements from front to back.

        Returns:
            An iterator over the elements
        """
        return iter(self._items)
