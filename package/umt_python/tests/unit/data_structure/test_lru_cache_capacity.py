from src.data_structure import LRUCache


def test_zero_capacity_keeps_only_the_latest_entry():
    cache = LRUCache[str, int](0)
    cache.set("a", 1)
    assert cache.size == 1
    assert cache.get("a") == 1

    cache.set("b", 2)
    assert cache.size == 1
    assert cache.has("a") is False
    assert cache.get("b") == 2


def test_negative_capacity_keeps_only_the_latest_entry():
    cache = LRUCache[str, int](-1)
    cache.set("a", 1)
    cache.set("b", 2)
    assert cache.size == 1
    assert cache.get("b") == 2


def test_updating_existing_key_at_capacity_does_not_evict():
    cache = LRUCache[str, int](2)
    cache.set("a", 1)
    cache.set("b", 2)
    cache.set("a", 10)
    assert cache.size == 2
    assert cache.get("a") == 10
    assert cache.get("b") == 2


def test_updating_existing_key_refreshes_recency():
    cache = LRUCache[str, int](2)
    cache.set("a", 1)
    cache.set("b", 2)
    cache.set("a", 10)
    cache.set("c", 3)
    assert cache.has("b") is False
    assert cache.get("a") == 10
    assert cache.get("c") == 3


def test_new_key_below_capacity_does_not_evict():
    cache = LRUCache[str, int](3)
    cache.set("a", 1)
    cache.set("b", 2)
    assert cache.size == 2
    assert cache.has("a") is True
