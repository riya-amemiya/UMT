from collections import OrderedDict
from typing import Generic, TypeVar

K = TypeVar("K")
V = TypeVar("V")


class LRUCache(Generic[K, V]):
    """
    A Least Recently Used (LRU) cache implementation
    using an OrderedDict for O(1) get/set operations.

    When the cache exceeds its capacity, the least recently used
    entry is evicted.

    Features:
    - get(key): Retrieve a value (moves it to most recently used)
    - set(key, value): Insert or update a value
    - has(key): Check if a key exists
    - delete(key): Remove a specific entry
    - clear(): Remove all entries
    - size: Get the number of entries
    """

    def __init__(self, capacity: int) -> None:
        """
        Creates a new LRUCache instance.

        Args:
            capacity: The maximum number of entries the cache can hold.
        """
        self._capacity = capacity
        self._cache: OrderedDict[K, V] = OrderedDict()

    @property
    def size(self) -> int:
        """
        Returns the number of entries in the cache.
        """
        return len(self._cache)

    def get(self, key: K) -> V | None:
        """
        Retrieves a value by key and marks it as most recently used.

        Args:
            key: The key to look up.

        Returns:
            The value if found, or None if not in cache.
        """
        if key not in self._cache:
            return None
        self._cache.move_to_end(key)
        return self._cache[key]

    def set(self, key: K, value: V) -> None:
        """
        Inserts or updates a key-value pair.
        If the cache is at capacity, the least recently used entry is evicted.

        Args:
            key: The key to set.
            value: The value to associate with the key.
        """
        if key in self._cache:
            self._cache.move_to_end(key)
        self._cache[key] = value
        if len(self._cache) > self._capacity:
            self._cache.popitem(last=False)

    def has(self, key: K) -> bool:
        """
        Checks whether a key exists in the cache.
        Does not affect the recently-used order.

        Args:
            key: The key to check.

        Returns:
            True if the key exists in the cache.
        """
        return key in self._cache

    def delete(self, key: K) -> bool:
        """
        Removes an entry from the cache by key.

        Args:
            key: The key to remove.

        Returns:
            True if the entry was found and removed.
        """
        if key in self._cache:
            del self._cache[key]
            return True
        return False

    def clear(self) -> None:
        """
        Removes all entries from the cache.
        """
        self._cache.clear()
