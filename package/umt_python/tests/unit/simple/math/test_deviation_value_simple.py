import pytest

from src.simple.math.deviation_value_simple import deviation_value_simple


def test_deviation_value_simple_explicit_values():
    assert deviation_value_simple(50, 50, 10) == 50
    assert deviation_value_simple(60, 50, 10) == 60
    assert deviation_value_simple(40, 50, 10) == 40
    assert deviation_value_simple(100, 50, 0) == 50

    with pytest.raises(TypeError):
        deviation_value_simple(50, 50)  # type: ignore


def test_deviation_value_simple_array_input():
    scores = [40, 50, 60]
    assert deviation_value_simple(60, scores) == pytest.approx(62.25, abs=0.01)
    assert deviation_value_simple(50, scores) == 50
    assert deviation_value_simple(40, scores) == pytest.approx(37.75, abs=0.01)

    same_scores = [50, 50, 50]
    assert deviation_value_simple(50, same_scores) == 50
    assert deviation_value_simple(0, same_scores) == 50
    assert deviation_value_simple(100, same_scores) == 50
