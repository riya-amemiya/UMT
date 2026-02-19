import copy

from src.object.get_objects_diff import get_objects_diff


def test_should_find_diff_key_value_pairs_between_two_objects():
    assert get_objects_diff({"a": 1, "b": 2}, {"b": 2, "c": 3}) == {"a": 1, "c": 3}


def test_should_find_diff_key_value_pairs_among_three_objects():
    assert get_objects_diff(
        {"a": 1, "b": 2, "c": 3}, {"b": 2, "c": 3, "d": 4}, {"c": 3, "d": 4, "e": 5}
    ) == {"a": 1, "e": 5}


def test_should_return_a_shallow_copy_for_a_single_object():
    obj = {"a": 1, "b": 2}
    result = get_objects_diff(obj)
    assert result == {"a": 1, "b": 2}
    assert result is not obj


def test_should_return_empty_object_when_objects_are_identical():
    assert get_objects_diff({"a": 1, "b": 2}, {"a": 1, "b": 2}) == {}


def test_should_return_all_pairs_when_no_keys_overlap():
    assert get_objects_diff({"a": 1}, {"b": 2}, {"c": 3}) == {"a": 1, "b": 2, "c": 3}


def test_should_handle_empty_objects():
    assert get_objects_diff({}, {"a": 1}) == {"a": 1}
    assert get_objects_diff({"a": 1}, {}) == {"a": 1}
    assert get_objects_diff({}, {}) == {}


def test_should_use_last_value_when_same_key_has_different_unique_values():
    assert get_objects_diff({"a": 1}, {"a": 2}) == {"a": 2}


def test_should_handle_same_key_with_shared_and_unique_values_across_three_objects():
    assert get_objects_diff({"a": 1}, {"a": 1}, {"a": 2}) == {"a": 2}


def test_should_handle_falsy_values_correctly():
    assert get_objects_diff({"a": None, "b": 0}, {"a": None, "b": 0}) == {}


def test_should_handle_different_falsy_values():
    # In JS: null !== undefined. In Python we can test False vs 0 which are == but not strict equal
    # strict equal (False, 0) should be False.
    # So they are unique.
    assert get_objects_diff({"a": False}, {"a": 0}) == {"a": 0}


def test_should_find_diff_in_nested_objects_recursively():
    assert get_objects_diff(
        {"a": {"b": 1, "c": 2}, "d": 3}, {"a": {"b": 1, "d": 4}, "d": 3}
    ) == {"a": {"c": 2, "d": 4}}


def test_should_exclude_key_when_nested_diff_is_empty():
    assert get_objects_diff({"a": {"b": 1}}, {"a": {"b": 1}}) == {}


def test_should_handle_deeply_nested_objects():
    assert get_objects_diff(
        {"a": {"b": {"c": {"d": 1, "e": 2}}}}, {"a": {"b": {"c": {"d": 1, "f": 3}}}}
    ) == {"a": {"b": {"c": {"e": 2, "f": 3}}}}


def test_should_handle_nested_objects_among_three_objects():
    assert get_objects_diff(
        {"a": {"b": 1, "c": 2}}, {"a": {"b": 1, "d": 3}}, {"a": {"b": 1, "e": 4}}
    ) == {"a": {"c": 2, "d": 3, "e": 4}}


def test_should_handle_nested_shared_values_across_three_objects():
    assert get_objects_diff({"a": {"b": 1}}, {"a": {"b": 1}}, {"a": {"b": 2}}) == {
        "a": {"b": 2}
    }


def test_should_handle_mixed_nested_and_primitive_values():
    assert get_objects_diff({"a": {"b": 1}}, {"a": "hello"}) == {"a": "hello"}


def test_should_compare_arrays_by_reference():
    arr = [1, 2, 3]
    assert get_objects_diff({"a": arr}, {"a": arr}) == {}
    # Different references should differ
    assert get_objects_diff({"a": [1, 2, 3]}, {"a": [1, 2, 3]}) == {"a": [1, 2, 3]}


def test_should_not_mutate_original_objects():
    obj1 = {"a": 1, "b": {"c": 2}}
    obj2 = {"a": 1, "b": {"c": 2, "d": 3}}
    obj1_copy = copy.deepcopy(obj1)
    obj2_copy = copy.deepcopy(obj2)

    get_objects_diff(obj1, obj2)

    assert obj1 == obj1_copy
    assert obj2 == obj2_copy
