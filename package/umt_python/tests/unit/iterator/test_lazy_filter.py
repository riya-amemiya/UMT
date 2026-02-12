from src.iterator import lazy_filter


class TestLazyFilter:
    def test_filters_values_lazily(self):
        result = list(lazy_filter([1, 2, 3, 4, 5], lambda n, _: n % 2 == 0))
        assert result == [2, 4]

    def test_provides_index_to_the_predicate(self):
        result = list(lazy_filter([10, 20, 30, 40], lambda _, i: i >= 2))
        assert result == [30, 40]

    def test_handles_empty_iterable(self):
        empty_list: list[int] = []
        result = list(lazy_filter(empty_list, lambda n, _: n > 0))
        assert result == []

    def test_handles_no_matching_elements(self):
        result = list(lazy_filter([1, 2, 3], lambda _, __: False))
        assert result == []

    def test_handles_all_matching_elements(self):
        result = list(lazy_filter([1, 2, 3], lambda _, __: True))
        assert result == [1, 2, 3]

    def test_evaluates_lazily(self):
        call_count = 0

        def predicate(n, _):
            nonlocal call_count
            call_count += 1
            return n > 3

        gen = lazy_filter([1, 2, 3, 4, 5], predicate)

        assert call_count == 0
        next(gen)
        assert call_count > 0

    def test_works_with_set_as_iterable(self):
        s = {1, 2, 3, 4, 5}
        result = sorted(list(lazy_filter(s, lambda n, _: n > 3)))
        assert result == [4, 5]
