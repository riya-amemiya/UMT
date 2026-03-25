from src.predicate import is_not_nullish


class TestIsNotNullish:
    def test_returns_false_for_none(self):
        assert is_not_nullish(None) is False

    def test_returns_true_for_zero(self):
        assert is_not_nullish(0) is True

    def test_returns_true_for_empty_string(self):
        assert is_not_nullish("") is True

    def test_returns_true_for_false(self):
        assert is_not_nullish(False) is True

    def test_returns_true_for_nan(self):
        assert is_not_nullish(float("nan")) is True

    def test_returns_true_for_objects(self):
        assert is_not_nullish({}) is True
        assert is_not_nullish([]) is True

    def test_returns_true_for_functions(self):
        assert is_not_nullish(lambda: None) is True

    def test_returns_true_for_numbers(self):
        assert is_not_nullish(42) is True
        assert is_not_nullish(3.14) is True

    def test_returns_true_for_strings(self):
        assert is_not_nullish("hello") is True
