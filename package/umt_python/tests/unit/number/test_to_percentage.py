from src.number.to_percentage import to_percentage


def test_calculate_basic_percentage():
    assert to_percentage(25, 100) == 25.0
    assert to_percentage(50, 100) == 50.0
    assert to_percentage(100, 100) == 100.0


def test_return_result_with_2_decimal_places_by_default():
    assert to_percentage(1, 3) == 33.33
    assert to_percentage(2, 3) == 66.67


def test_respect_custom_decimal_places():
    assert to_percentage(1, 3, 0) == 33.0
    assert to_percentage(1, 3, 1) == 33.3
    assert to_percentage(1, 3, 4) == 33.3333


def test_return_0_when_total_is_0():
    assert to_percentage(0, 0) == 0.0
    assert to_percentage(5, 0) == 0.0
    assert to_percentage(-5, 0) == 0.0


def test_handle_0_value():
    assert to_percentage(0, 100) == 0.0


def test_handle_values_greater_than_total():
    assert to_percentage(150, 100) == 150.0
    assert to_percentage(200, 100) == 200.0


def test_handle_negative_values():
    assert to_percentage(-25, 100) == -25.0


def test_handle_negative_half_rounding():
    # JS Math.round(-1.5) -> -1
    # JS Math.round(-2.5) -> -2

    # -1.5 / 1 * 100 * 1 = -150? No.
    # to_percentage(val, total, decimals)
    # val=-1.5, total=1, decimals=0
    # (-1.5 / 1) * 100 = -150.
    # factor = 1.
    # round(-150) = -150.
    # result = -150.

    # Wait, to test rounding behavior I need the number just before rounding to end in .5
    # e.g. result before rounding is -1.5

    # val = -0.015, total = 1.
    # (-0.015 / 1) * 100 = -1.5.
    # factor = 1.
    # round(-1.5) = -1.
    assert to_percentage(-0.015, 1, 0) == -1.0

    # val = -0.025, total = 1.
    # (-0.025 / 1) * 100 = -2.5.
    # factor = 1.
    # round(-2.5) = -2.
    assert to_percentage(-0.025, 1, 0) == -2.0

    # Test .5 rounding behavior for positive numbers
    # val = 0.015, total = 1.
    # (0.015 / 1) * 100 = 1.5.
    # round(1.5) = 2.
    assert to_percentage(0.015, 1, 0) == 2.0

    # val = 0.025, total = 1.
    # (0.025 / 1) * 100 = 2.5.
    # round(2.5) = 3.
    assert to_percentage(0.025, 1, 0) == 3.0
