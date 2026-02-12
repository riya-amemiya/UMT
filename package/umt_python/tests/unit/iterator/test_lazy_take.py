from src.iterator import lazy_take


class TestLazyTake:
    def test_takes_the_first_n_elements(self):
        result = list(lazy_take([1, 2, 3, 4, 5], 3))
        assert result == [1, 2, 3]

    def test_takes_all_when_n_exceeds_length(self):
        result = list(lazy_take([1, 2], 5))
        assert result == [1, 2]

    def test_takes_nothing_when_n_is_0(self):
        result = list(lazy_take([1, 2, 3], 0))
        assert result == []

    def test_handles_empty_iterable(self):
        result = list(lazy_take([], 5))
        assert result == []

    def test_stops_iterating_after_n_elements(self):
        yield_count = 0

        def source():
            nonlocal yield_count
            for i in range(100):
                yield_count += 1
                yield i

        result = list(lazy_take(source(), 3))
        assert result == [0, 1, 2]
        assert yield_count == 3

    def test_works_with_set_as_iterable(self):
        s = {10, 20, 30, 40, 50}
        result = list(lazy_take(s, 2))
        assert len(result) == 2
        for x in result:
            assert x in s

    def test_chains_with_other_lazy_operations(self):
        def naturals():
            n = 1
            while True:
                yield n
                n += 1

        result = list(lazy_take(naturals(), 5))
        assert result == [1, 2, 3, 4, 5]

    def test_does_not_consume_extra_element(self):
        def source():
            yield 1
            yield 2
            yield 3
            yield 4

        gen = source()
        result = list(lazy_take(gen, 2))
        assert result == [1, 2]
        assert next(gen) == 3
