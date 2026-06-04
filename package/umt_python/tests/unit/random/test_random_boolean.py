from src.random import random_boolean


class TestRandomBoolean:
    def test_returns_a_boolean(self):
        assert isinstance(random_boolean(), bool)

    def test_returns_true_when_probability_is_1(self):
        for _ in range(50):
            assert random_boolean(1) is True

    def test_returns_false_when_probability_is_0(self):
        for _ in range(50):
            assert random_boolean(0) is False

    def test_respects_probability_bias_roughly(self):
        true_count = 0
        trials = 2000
        for _ in range(trials):
            if random_boolean(0.9):
                true_count += 1
        assert true_count / trials > 0.7
