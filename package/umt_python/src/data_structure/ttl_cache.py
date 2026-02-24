import time
from collections import OrderedDict
from typing import Any, Generic, TypeVar

K = TypeVar("K")
V = TypeVar("V")


class TTLCache(Generic[K, V]):
    """
    A time-to-live (TTL) cache that automatically expires entries
    after a configured duration.

    Uses lazy deletion: expired entries are only removed
    when accessed via get() or has().

    Features:
    - get(key): Retrieve a value (returns None if expired)
    - set(key, value, ttl?): Insert with optional per-entry TTL
    - has(key): Check if a non-expired key exists
    - delete(key): Remove a specific entry
    - clear(): Remove all entries
    - size: Get the number of entries (including expired)
    """

    def __init__(self, default_ttl: int, max_size: int | None = None) -> None:
        """
        Creates a new TTLCache instance.

        Args:
            default_ttl: Default time-to-live in milliseconds for cache entries.
            max_size: Optional maximum number of entries.
        """
        self.default_ttl = default_ttl
        self.max_size = max_size
        self._cache: OrderedDict[K, dict[str, Any]] = OrderedDict()

    @property
    def size(self) -> int:
        """
        Returns the number of entries in the cache
        (including potentially expired entries).
        """
        return len(self._cache)

    def get(self, key: K) -> V | None:
        """
        Retrieves a value by key. Returns None if the key
        does not exist or has expired.

        Args:
            key: The key to look up.

        Returns:
            The value if found and not expired, or None.
        """
        entry = self._cache.get(key)
        if entry is None:
            return None

        if time.time() * 1000 >= entry["expires_at"]:
            del self._cache[key]
            return None

        return entry["value"]

    def set(self, key: K, value: V, ttl: int | None = None) -> None:
        """
        Inserts or updates a key-value pair with an optional TTL override.
        If max_size is configured and the cache is full, the oldest entry
        is removed.

        Args:
            key: The key to set.
            value: The value to cache.
            ttl: Optional TTL in milliseconds (overrides default_ttl).
        """
        effective_ttl = ttl if ttl is not None else self.default_ttl
        expires_at = time.time() * 1000 + effective_ttl

        if key in self._cache:
            self._cache[key] = {"value": value, "expires_at": expires_at}
            return

        if self.max_size is not None and len(self._cache) >= self.max_size:
            self._cache.popitem(last=False)

        self._cache[key] = {"value": value, "expires_at": expires_at}

    def has(self, key: K) -> bool:
        """
        Checks whether a non-expired key exists in the cache.
        Removes the entry if it has expired.

        Args:
            key: The key to check.

        Returns:
            True if the key exists and has not expired.
        """
        entry = self._cache.get(key)
        if entry is None:
            return False

        if time.time() * 1000 >= entry["expires_at"]:
            del self._cache[key]
            return False

        return True

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
