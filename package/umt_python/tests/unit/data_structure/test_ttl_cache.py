from unittest.mock import patch

from src.data_structure import TTLCache


def test_constructor():
    cache = TTLCache[str, int](default_ttl=5000)
    assert cache.size == 0


def test_set_and_get():
    cache = TTLCache[str, int](default_ttl=5000)
    cache.set("a", 1)
    cache.set("b", 2)

    assert cache.get("a") == 1
    assert cache.get("b") == 2


def test_get_missing_key():
    cache = TTLCache[str, int](default_ttl=5000)
    assert cache.get("missing") is None


def test_update_existing_key():
    cache = TTLCache[str, int](default_ttl=5000)
    cache.set("a", 1)
    cache.set("a", 10)
    assert cache.get("a") == 10
    assert cache.size == 1


@patch("time.time")
def test_ttl_expiration(mock_time):
    mock_time.return_value = 1000.0  # Start time in seconds

    cache = TTLCache[str, int](default_ttl=5000)
    cache.set("a", 1)
    assert cache.get("a") == 1

    # Advance time by 5 seconds (5000ms)
    mock_time.return_value = 1005.0

    assert cache.get("a") is None


@patch("time.time")
def test_not_expire_before_ttl(mock_time):
    mock_time.return_value = 1000.0

    cache = TTLCache[str, int](default_ttl=5000)
    cache.set("a", 1)

    # Advance time by 4.999 seconds
    mock_time.return_value = 1004.999

    assert cache.get("a") == 1


@patch("time.time")
def test_per_entry_ttl_override(mock_time):
    mock_time.return_value = 1000.0

    cache = TTLCache[str, int](default_ttl=5000)
    cache.set("short", 1, ttl=1000)
    cache.set("long", 2, ttl=10000)

    # Advance 1 second
    mock_time.return_value = 1001.0
    assert cache.get("short") is None
    assert cache.get("long") == 2

    # Advance another 9 seconds (total 10s from start)
    mock_time.return_value = 1010.0
    assert cache.get("long") is None


@patch("time.time")
def test_expire_entries_checked_via_has(mock_time):
    mock_time.return_value = 1000.0

    cache = TTLCache[str, int](default_ttl=5000)
    cache.set("a", 1)
    assert cache.has("a") is True

    # Advance 5 seconds
    mock_time.return_value = 1005.0
    assert cache.has("a") is False


@patch("time.time")
def test_cleanup_expired_entries_on_get(mock_time):
    mock_time.return_value = 1000.0

    cache = TTLCache[str, int](default_ttl=5000)
    cache.set("a", 1)

    # Advance 5 seconds
    mock_time.return_value = 1005.0

    # Before get, size is 1 (lazy deletion)
    assert cache.size == 1
    cache.get("a")
    assert cache.size == 0


@patch("time.time")
def test_cleanup_expired_entries_on_has(mock_time):
    mock_time.return_value = 1000.0

    cache = TTLCache[str, int](default_ttl=5000)
    cache.set("a", 1)

    # Advance 5 seconds
    mock_time.return_value = 1005.0

    assert cache.size == 1
    cache.has("a")
    assert cache.size == 0


def test_max_size_eviction():
    cache = TTLCache[str, int](default_ttl=5000, max_size=2)
    cache.set("a", 1)
    cache.set("b", 2)
    cache.set("c", 3)

    assert cache.size == 2
    assert cache.get("a") is None
    assert cache.get("b") == 2
    assert cache.get("c") == 3


def test_no_eviction_when_updating_existing_key():
    cache = TTLCache[str, int](default_ttl=5000, max_size=2)
    cache.set("a", 1)
    cache.set("b", 2)
    cache.set("a", 10)  # Updates 'a', but 'a' is still oldest inserted

    assert cache.size == 2
    assert cache.get("a") == 10
    assert cache.get("b") == 2

    # Add 'c', 'a' should be evicted because it's the oldest inserted
    cache.set("c", 3)
    assert cache.get("a") is None
    assert cache.get("b") == 2
    assert cache.get("c") == 3


def test_has():
    cache = TTLCache[str, int](default_ttl=5000)
    cache.set("a", 1)
    assert cache.has("a") is True
    assert cache.has("missing") is False


def test_delete():
    cache = TTLCache[str, int](default_ttl=5000)
    cache.set("a", 1)
    assert cache.delete("a") is True
    assert cache.has("a") is False
    assert cache.size == 0
    assert cache.delete("missing") is False


def test_clear():
    cache = TTLCache[str, int](default_ttl=5000)
    cache.set("a", 1)
    cache.set("b", 2)
    cache.clear()
    assert cache.size == 0
    assert cache.get("a") is None


def test_max_size_1():
    cache = TTLCache[str, int](default_ttl=5000, max_size=1)
    cache.set("a", 1)
    cache.set("b", 2)
    assert cache.size == 1
    assert cache.get("a") is None
    assert cache.get("b") == 2


@patch("time.time")
def test_ttl_zero(mock_time):
    mock_time.return_value = 1000.0
    cache = TTLCache[str, int](default_ttl=0)
    cache.set("a", 1)

    assert cache.get("a") is None


def test_numeric_keys():
    cache = TTLCache[int, str](default_ttl=5000)
    cache.set(1, "one")
    cache.set(2, "two")
    assert cache.get(1) == "one"
    assert cache.get(2) == "two"
