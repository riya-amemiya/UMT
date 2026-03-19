import pytest
from src.object.object_key_by import object_key_by


def test_object_key_by_property_name():
    input_data = [
        {"id": "a1", "name": "Alice"},
        {"id": "b2", "name": "Bob"},
    ]
    expected = {
        "a1": {"id": "a1", "name": "Alice"},
        "b2": {"id": "b2", "name": "Bob"},
    }
    assert object_key_by(input_data, "id") == expected


def test_object_key_by_custom_function():
    input_data = [
        {"dir": "left", "code": 97},
        {"dir": "right", "code": 100},
    ]
    expected = {
        "a": {"dir": "left", "code": 97},
        "d": {"dir": "right", "code": 100},
    }
    assert object_key_by(input_data, lambda o: chr(o["code"])) == expected


def test_object_key_by_empty_array():
    assert object_key_by([], "id") == {}


def test_object_key_by_duplicate_keys():
    input_data = [
        {"id": "a1", "name": "Alice"},
        {"id": "a1", "name": "Alex"},
    ]
    expected = {
        "a1": {"id": "a1", "name": "Alex"},
    }
    assert object_key_by(input_data, "id") == expected


def test_object_key_by_object_input():
    input_data = {
        "first": {"id": "a1", "name": "Alice"},
        "second": {"id": "b2", "name": "Bob"},
    }
    expected = {
        "a1": {"id": "a1", "name": "Alice"},
        "b2": {"id": "b2", "name": "Bob"},
    }
    assert object_key_by(input_data, "id") == expected


def test_object_key_by_no_iteratee():
    input_data = ["a", "b"]
    expected = {
        "a": "a",
        "b": "b",
    }
    assert object_key_by(input_data) == expected
