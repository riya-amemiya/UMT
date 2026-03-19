from src.predicate.matches import matches


class TestMatches:
    def test_returns_true_when_object_matches_the_pattern(self) -> None:
        is_admin = matches({"role": "admin"})
        assert is_admin({"name": "Alice", "role": "admin"}) is True

    def test_returns_false_when_object_does_not_match(self) -> None:
        is_admin = matches({"role": "admin"})
        assert is_admin({"name": "Bob", "role": "user"}) is False

    def test_matches_multiple_properties(self) -> None:
        matcher = matches({"a": 1, "b": 2})
        assert matcher({"a": 1, "b": 2, "c": 3}) is True
        assert matcher({"a": 1, "b": 3}) is False
        assert matcher({"a": 2, "b": 2}) is False

    def test_uses_strict_equality(self) -> None:
        matcher = matches({"value": 0})
        assert matcher({"value": 0}) is True
        assert matcher({"value": False}) is False
        assert matcher({"value": ""}) is False
        assert matcher({"value": None}) is False

    def test_returns_true_for_empty_pattern(self) -> None:
        always_match = matches({})
        assert always_match({"anything": "goes"}) is True
        assert always_match({}) is True

    def test_handles_missing_keys_in_target_object(self) -> None:
        matcher = matches({"x": 1})
        assert matcher({}) is False
        assert matcher({"y": 1}) is False

    def test_handles_none_pattern_values(self) -> None:
        match_none = matches({"a": None})
        assert match_none({"a": None}) is True
        assert match_none({"a": False}) is False
        assert match_none({"a": 0}) is False
