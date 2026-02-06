from src.simple.array.quick_sort_simple import quick_sort_simple


def test_returns_empty_array_when_sorting_empty_array():
    assert quick_sort_simple([]) == []


def test_handles_start_id_outside_array_bounds():
    assert quick_sort_simple([3, 1, 4], None, -1, 2) == [1, 3, 4]
    assert quick_sort_simple([3, 1, 4], None, 4, 2) == [1, 3, 4]


def test_handles_end_id_outside_array_bounds():
    assert quick_sort_simple([3, 1, 4], None, 0, 5) == [1, 3, 4]


def test_handles_start_id_greater_than_end_id():
    assert quick_sort_simple([3, 1, 4], None, 2, 1) == [1, 3, 4]
