from src.random import random_int


class TestRandomInt:
    def test_returns_integers_within_the_closed_interval(self):
        for _ in range(200):
            value = random_int(1, 6)
            assert isinstance(value, int)
            assert value >= 1
            assert value <= 6

    def test_returns_the_bound_when_min_equals_max(self):
        assert random_int(3, 3) == 3

    def test_supports_negative_ranges(self):
        for _ in range(50):
            value = random_int(-3, 3)
            assert value >= -3
            assert value <= 3

    def test_ceils_the_lower_bound_and_floors_the_upper_bound(self):
        for _ in range(50):
            value = random_int(0.4, 5.7)
            assert value >= 1
            assert value <= 5
