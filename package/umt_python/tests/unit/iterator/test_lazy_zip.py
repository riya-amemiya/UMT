from src.iterator import lazy_zip


class TestLazyZip:
    def test_combines_values(self):
        assert list(lazy_zip([1, 2, 3], ["a", "b", "c"])) == [
            [1, "a"],
            [2, "b"],
            [3, "c"],
        ]

    def test_stops_at_shortest(self):
        assert list(lazy_zip([1, 2, 3], ["a", "b"], [True])) == [[1, "a", True]]

    def test_no_iterables(self):
        assert list(lazy_zip()) == []

    def test_single_iterable(self):
        assert list(lazy_zip((1, 2))) == [[1], [2]]

    def test_empty_iterable(self):
        assert list(lazy_zip([1, 2], [])) == []

    def test_works_with_infinite_iterables(self):
        def naturals():
            n = 0
            while True:
                yield n
                n += 1

        assert list(lazy_zip(naturals(), ["a", "b"])) == [[0, "a"], [1, "b"]]

    def test_closes_sources_when_exhausted(self):
        closed: list[str] = []

        def tracked(name: str, values: list[int]):
            try:
                yield from values
            finally:
                closed.append(name)

        result = list(lazy_zip(tracked("first", [1, 2, 3]), tracked("second", [4])))
        assert result == [[1, 4]]
        assert sorted(closed) == ["first", "second"]

    def test_closes_sources_when_consumer_stops(self):
        closed: list[str] = []

        def tracked(name: str):
            try:
                n = 0
                while True:
                    yield n
                    n += 1
            finally:
                closed.append(name)

        gen = lazy_zip(tracked("first"), tracked("second"))
        assert next(gen) == [0, 0]
        gen.close()
        assert sorted(closed) == ["first", "second"]
