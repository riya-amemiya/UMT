from src.data_structure import LRUCache


def test_constructor():
    cache = LRUCache[str, int](3)
    assert cache.size == 0


def test_set_and_get():
    cache = LRUCache[str, int](3)
    cache.set("a", 1)
    cache.set("b", 2)
    cache.set("c", 3)

    assert cache.get("a") == 1
    assert cache.get("b") == 2
    assert cache.get("c") == 3


def test_get_missing():
    cache = LRUCache[str, int](3)
    assert cache.get("missing") is None


def test_update_existing_key():
    cache = LRUCache[str, int](3)
    cache.set("a", 1)
    cache.set("a", 10)
    assert cache.get("a") == 10
    assert cache.size == 1


def test_eviction():
    cache = LRUCache[str, int](3)
    cache.set("a", 1)
    cache.set("b", 2)
    cache.set("c", 3)
    cache.set("d", 4)

    assert cache.has("a") is False
    assert cache.get("b") == 2
    assert cache.get("c") == 3
    assert cache.get("d") == 4
    assert cache.size == 3


def test_promote_accessed_entries():
    cache = LRUCache[str, int](3)
    cache.set("a", 1)
    cache.set("b", 2)
    cache.set("c", 3)

    cache.get("a")
    cache.set("d", 4)

    assert cache.has("a") is True
    assert cache.has("b") is False
    assert cache.has("c") is True
    assert cache.has("d") is True


def test_promote_updated_entries():
    cache = LRUCache[str, int](3)
    cache.set("a", 1)
    cache.set("b", 2)
    cache.set("c", 3)

    cache.set("a", 10)
    cache.set("d", 4)

    assert cache.has("a") is True
    assert cache.get("a") == 10
    assert cache.has("b") is False


def test_has():
    cache = LRUCache[str, int](3)
    cache.set("a", 1)
    assert cache.has("a") is True
    assert cache.has("missing") is False


def test_delete():
    cache = LRUCache[str, int](3)
    cache.set("a", 1)
    cache.set("b", 2)

    assert cache.delete("a") is True
    assert cache.has("a") is False
    assert cache.size == 1


def test_delete_missing():
    cache = LRUCache[str, int](3)
    assert cache.delete("missing") is False


def test_delete_head_node():
    # In this context, "head" refers to MRU in TS implementation terms,
    # or just ensuring we can delete any node correctly.
    cache = LRUCache[str, int](3)
    cache.set("a", 1)
    cache.set("b", 2)
    cache.set("c", 3)

    assert cache.delete("c") is True
    assert cache.size == 2
    assert cache.get("a") == 1
    assert cache.get("b") == 2


def test_delete_tail_node():
    # In this context, "tail" refers to LRU in TS implementation terms.
    cache = LRUCache[str, int](3)
    cache.set("a", 1)
    cache.set("b", 2)
    cache.set("c", 3)

    assert cache.delete("a") is True
    assert cache.size == 2
    assert cache.get("b") == 2
    assert cache.get("c") == 3


def test_delete_only_entry():
    cache = LRUCache[str, int](3)
    cache.set("a", 1)
    assert cache.delete("a") is True
    assert cache.size == 0


def test_clear():
    cache = LRUCache[str, int](3)
    cache.set("a", 1)
    cache.set("b", 2)
    cache.set("c", 3)

    cache.clear()

    assert cache.size == 0
    assert cache.get("a") is None
    assert cache.get("b") is None
    assert cache.get("c") is None


def test_size_tracking():
    cache = LRUCache[str, int](5)
    assert cache.size == 0

    cache.set("a", 1)
    assert cache.size == 1

    cache.set("b", 2)
    assert cache.size == 2

    cache.delete("a")
    assert cache.size == 1


def test_capacity_one():
    cache = LRUCache[str, int](1)
    cache.set("a", 1)
    assert cache.get("a") == 1

    cache.set("b", 2)
    assert cache.get("a") is None
    assert cache.get("b") == 2
    assert cache.size == 1


def test_numeric_keys():
    cache = LRUCache[int, str](3)
    cache.set(1, "one")
    cache.set(2, "two")
    cache.set(3, "three")

    assert cache.get(1) == "one"
    assert cache.get(2) == "two"
    assert cache.get(3) == "three"


def test_many_operations():
    cache = LRUCache[int, int](100)
    for i in range(1000):
        cache.set(i, i * 2)

    assert cache.size == 100

    for i in range(900, 1000):
        assert cache.get(i) == i * 2

    assert cache.get(0) is None
