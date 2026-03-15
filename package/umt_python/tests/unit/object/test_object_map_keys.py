from src.object.object_map_keys import map_keys

class TestMapKeys:
    def test_transform_keys_using_provided_function(self):
        result = map_keys({"a": 1, "b": 2, "c": 3}, lambda _value, key: key.upper())
        assert result == {"A": 1, "B": 2, "C": 3}

    def test_pass_value_and_key_to_transformer(self):
        result = map_keys({"x": 10, "y": 20}, lambda value, key: f"{key}_{value}")
        assert result == {"x_10": 10, "y_20": 20}

    def test_handle_empty_object(self):
        result = map_keys({}, lambda _value, key: key)
        assert result == {}

    def test_handle_keys_that_map_to_same_value(self):
        result = map_keys({"a": 1, "b": 2}, lambda _value, _key: "same")
        assert result == {"same": 2}

    def test_not_modify_original_object(self):
        original = {"a": 1, "b": 2}
        map_keys(original, lambda _value, key: key.upper())
        assert original == {"a": 1, "b": 2}
