from src.predicate import is_nullish


class TestIsNullish:
    def test_returns_true_for_none(self):
        assert is_nullish(None) is True

    def test_returns_false_for_zero(self):
        assert is_nullish(0) is False

    def test_returns_false_for_empty_string(self):
        assert is_nullish("") is False

    def test_returns_false_for_false(self):
        assert is_nullish(False) is False

    def test_returns_false_for_nan(self):
        assert is_nullish(float("nan")) is False

    def test_returns_false_for_objects(self):
        assert is_nullish({}) is False
        assert is_nullish([]) is False

    def test_returns_false_for_functions(self):
        assert is_nullish(lambda: None) is False

    def test_returns_false_for_numbers(self):
        assert is_nullish(42) is False
        assert is_nullish(3.14) is False

    def test_returns_false_for_strings(self):
        assert is_nullish("hello") is False
