from src.random import seeded_random


class TestSeededRandom:
    def test_produces_values_in_0_1(self):
        rand = seeded_random(42)
        for _ in range(100):
            value = rand()
            assert value >= 0
            assert value < 1

    def test_is_deterministic_for_the_same_numeric_seed(self):
        a = seeded_random(123)
        b = seeded_random(123)
        for _ in range(20):
            assert a() == b()

    def test_is_deterministic_for_the_same_string_seed(self):
        a = seeded_random("hello")
        b = seeded_random("hello")
        for _ in range(20):
            assert a() == b()

    def test_produces_different_streams_for_different_seeds(self):
        a = seeded_random(1)
        b = seeded_random(2)
        differs = False
        for _ in range(10):
            if a() != b():
                differs = True
                break
        assert differs is True

    def test_handles_a_numeric_seed_of_0(self):
        rand = seeded_random(0)
        for _ in range(20):
            value = rand()
            assert value >= 0
            assert value < 1

    def test_handles_an_empty_string_seed(self):
        rand = seeded_random("")
        value = rand()
        assert value >= 0
        assert value < 1
