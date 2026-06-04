from src.random import WeightedItem, weighted_choice


class TestWeightedChoice:
    def test_returns_one_of_the_weighted_values(self):
        items: list[WeightedItem] = [
            {"value": "a", "weight": 1},
            {"value": "b", "weight": 1},
        ]
        allowed = {"a", "b"}
        for _ in range(50):
            assert weighted_choice(items) in allowed

    def test_favors_heavier_weights_statistically(self):
        items: list[WeightedItem] = [
            {"value": "a", "weight": 1},
            {"value": "b", "weight": 9},
        ]
        b_count = 0
        trials = 2000
        for _ in range(trials):
            if weighted_choice(items) == "b":
                b_count += 1
        assert b_count / trials > 0.8

    def test_ignores_non_positive_weights(self):
        items: list[WeightedItem] = [
            {"value": "skip", "weight": 0},
            {"value": "winner", "weight": 5},
        ]
        for _ in range(50):
            assert weighted_choice(items) == "winner"

    def test_treats_negative_weights_as_ignored(self):
        items: list[WeightedItem] = [
            {"value": "a", "weight": -3},
            {"value": "b", "weight": 5},
        ]
        for _ in range(50):
            assert weighted_choice(items) == "b"
