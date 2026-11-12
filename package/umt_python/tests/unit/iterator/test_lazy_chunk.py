from src.iterator import lazy_chunk


class TestLazyChunk:
    def test_groups_values_into_chunks(self):
        assert list(lazy_chunk([1, 2, 3, 4, 5, 6], 2)) == [[1, 2], [3, 4], [5, 6]]

    def test_yields_shorter_last_chunk(self):
        assert list(lazy_chunk([1, 2, 3, 4, 5], 2)) == [[1, 2], [3, 4], [5]]

    def test_one_chunk_when_size_exceeds_length(self):
        assert list(lazy_chunk([1, 2], 5)) == [[1, 2]]

    def test_empty_iterable(self):
        assert list(lazy_chunk([], 3)) == []

    def test_yields_independent_lists(self):
        chunks = list(lazy_chunk([1, 2, 3, 4], 2))
        chunks[0].append(99)
        assert chunks[1] == [3, 4]

    def test_works_with_infinite_iterables(self):
        pulled = 0

        def naturals():
            nonlocal pulled
            n = 1
            while True:
                pulled += 1
                yield n
                n += 1

        gen = lazy_chunk(naturals(), 3)
        assert next(gen) == [1, 2, 3]
        assert next(gen) == [4, 5, 6]
        assert pulled == 6

    def test_works_with_strings(self):
        assert list(lazy_chunk("abc", 2)) == [["a", "b"], ["c"]]
