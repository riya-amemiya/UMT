from datetime import datetime
from src.object.get_objects_common import get_objects_common


def test_find_common_key_value_pairs_between_two_objects():
    assert get_objects_common({"a": 1, "b": 2}, {"a": 1, "c": 3}) == {"a": 1}


def test_find_common_key_value_pairs_among_three_objects():
    assert get_objects_common(
        {"a": 1, "b": 2, "c": 3},
        {"a": 1, "b": 2, "d": 4},
        {"a": 1, "e": 5},
    ) == {"a": 1}


def test_return_shallow_copy_for_single_object():
    obj = {"a": 1, "b": 2}
    result = get_objects_common(obj)
    assert result == obj
    assert result is not obj


def test_return_empty_object_when_one_input_is_empty():
    assert get_objects_common({"a": 1, "b": 2}, {}) == {}
    assert get_objects_common({}, {"a": 1, "b": 2}) == {}


def test_return_empty_object_when_all_inputs_are_empty():
    assert get_objects_common({}, {}) == {}


def test_return_empty_object_when_no_keys_are_common():
    assert get_objects_common({"a": 1}, {"b": 2}) == {}


def test_exclude_keys_with_different_values():
    assert get_objects_common({"a": 1, "b": 2}, {"a": 1, "b": 5}) == {"a": 1}


def test_handle_falsy_values_correctly():
    obj = {"a": None, "b": 0, "c": False, "d": ""}
    assert get_objects_common(obj, obj) == obj


def test_handle_falsy_values_that_differ():
    # 0 vs false
    assert get_objects_common({"a": 0}, {"a": False}) == {}
    # "" vs 0
    assert get_objects_common({"a": ""}, {"a": 0}) == {}
    # None vs False
    assert get_objects_common({"a": None}, {"a": False}) == {}


def test_find_common_nested_objects_recursively():
    assert get_objects_common(
        {"a": {"b": 1, "c": 2}, "d": 3},
        {"a": {"b": 1, "d": 4}, "d": 3},
    ) == {"a": {"b": 1}, "d": 3}


def test_exclude_key_when_recursive_result_is_empty():
    assert get_objects_common({"a": {"b": 1}}, {"a": {"c": 2}}) == {}


def test_handle_deeply_nested_objects():
    assert get_objects_common(
        {"a": {"b": {"c": {"d": 1, "e": 2}}}},
        {"a": {"b": {"c": {"d": 1, "f": 3}}}},
    ) == {"a": {"b": {"c": {"d": 1}}}}


def test_handle_nested_objects_among_three_objects():
    assert get_objects_common(
        {"a": {"b": 1, "c": 2, "d": 3}},
        {"a": {"b": 1, "c": 2, "e": 4}},
        {"a": {"b": 1, "f": 5}},
    ) == {"a": {"b": 1}}


def test_handle_mixed_nested_and_primitive_values():
    assert get_objects_common({"a": {"b": 1}}, {"a": "hello"}) == {}


def test_compare_arrays_by_reference():
    arr = [1, 2, 3]
    assert get_objects_common({"a": arr}, {"a": arr}) == {"a": arr}
    assert get_objects_common({"a": [1, 2, 3]}, {"a": [1, 2, 3]}) == {}


def test_compare_object_values_by_reference_when_not_plain_objects():
    date = datetime(2024, 1, 1)
    assert get_objects_common({"a": date}, {"a": date}) == {"a": date}
    assert (
        get_objects_common(
            {"a": datetime(2024, 1, 1)},
            {"a": datetime(2024, 1, 1)},
        )
        == {}
    )


def test_should_not_mutate_original_objects():
    obj1 = {"a": 1, "b": {"c": 2}}
    obj2 = {"a": 1, "b": {"c": 2, "d": 3}}
    obj1_copy = {"a": 1, "b": {"c": 2}}
    obj2_copy = {"a": 1, "b": {"c": 2, "d": 3}}

    get_objects_common(obj1, obj2)

    assert obj1 == obj1_copy
    assert obj2 == obj2_copy
