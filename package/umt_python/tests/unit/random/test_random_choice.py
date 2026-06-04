from src.random import random_choice


class TestRandomChoice:
    def test_returns_an_element_from_the_array(self):
        items = ["a", "b", "c"]
        for _ in range(50):
            assert random_choice(items) in items

    def test_returns_the_only_element_for_single_item_arrays(self):
        assert random_choice([42]) == 42
