from src.object import object_map_values


def test_map_values_basic_transform():
    result = object_map_values(
        {"a": 1, "b": 2, "c": 3},
        lambda value, _: value * 2,
    )
    assert result == {"a": 2, "b": 4, "c": 6}


def test_map_values_pass_value_and_key():
    result = object_map_values(
        {"x": 10, "y": 20},
        lambda value, key: f"{key}={value}",
    )
    assert result == {"x": "x=10", "y": "y=20"}


def test_map_values_empty_object():
    result = object_map_values({}, lambda value, _: value)
    assert result == {}


def test_map_values_not_modify_original():
    original = {"a": 1, "b": 2}
    object_map_values(original, lambda value, _: value * 2)
    assert original == {"a": 1, "b": 2}


def test_map_values_different_return_types():
    result = object_map_values(
        {"a": 1, "b": 2, "c": 3},
        lambda value, _: value > 1,
    )
    assert result == {"a": False, "b": True, "c": True}
