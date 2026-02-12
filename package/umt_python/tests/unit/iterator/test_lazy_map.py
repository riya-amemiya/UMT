from src.iterator import lazy_map


class TestLazyMap:
    def test_maps_values_lazily(self):
        result = list(lazy_map([1, 2, 3], lambda n, _: n * 2))
        assert result == [2, 4, 6]

    def test_provides_index_to_the_mapping_function(self):
        result = list(lazy_map(["a", "b", "c"], lambda v, i: f"{v}-{i}"))
        assert result == ["a-0", "b-1", "c-2"]

    def test_handles_empty_iterable(self):
        result = list(lazy_map([], lambda n, _: n))
        assert result == []

    def test_evaluates_lazily(self):
        call_count = 0

        def mapper(n, _):
            nonlocal call_count
            call_count += 1
            return n * 2

        gen = lazy_map([1, 2, 3, 4, 5], mapper)

        assert call_count == 0

        next(gen)
        assert call_count == 1

        next(gen)
        assert call_count == 2

    def test_works_with_set_as_iterable(self):
        s = {10, 20, 30}
        result = sorted(list(lazy_map(s, lambda n, _: n + 1)))
        assert result == [11, 21, 31]

    def test_works_with_string_as_iterable(self):
        result = list(lazy_map("abc", lambda c, _: c.upper()))
        assert result == ["A", "B", "C"]

    def test_works_with_generator_as_iterable(self):
        def source():
            yield 1
            yield 2
            yield 3

        result = list(lazy_map(source(), lambda n, _: n * 10))
        assert result == [10, 20, 30]
