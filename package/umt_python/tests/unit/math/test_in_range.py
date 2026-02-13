from src.math.in_range import in_range


def test_in_range_two_args():
    assert in_range(3, 5) is True
    assert in_range(5, 5) is False
    assert in_range(0, 5) is True
    assert in_range(-1, 5) is False


def test_in_range_three_args():
    assert in_range(3, 2, 5) is True
    assert in_range(3, 5, 2) is True
    assert in_range(5, 2, 5) is False
    assert in_range(2, 2, 5) is True


def test_in_range_negative():
    assert in_range(-3, -5, -1) is True
    assert in_range(-5, -5, -1) is True
    assert in_range(-1, -5, -1) is False


def test_in_range_float():
    assert in_range(0.5, 0, 1) is True
    assert in_range(1.5, 0, 1) is False


def test_in_range_negative_two_args():
    assert in_range(-3, -5) is True
    assert in_range(0, -5) is False
    assert in_range(-5, -5) is True
