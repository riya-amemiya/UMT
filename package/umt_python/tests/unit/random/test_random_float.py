from src.random import random_float


class TestRandomFloat:
    def test_returns_values_within_the_half_open_interval(self):
        for _ in range(100):
            value = random_float(5, 10)
            assert value >= 5
            assert value < 10

    def test_returns_the_lower_bound_when_min_equals_max(self):
        assert random_float(7, 7) == 7

    def test_supports_negative_ranges(self):
        for _ in range(50):
            value = random_float(-5, -1)
            assert value >= -5
            assert value < -1
