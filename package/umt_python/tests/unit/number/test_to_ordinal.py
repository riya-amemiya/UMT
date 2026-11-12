import math

from src.number import to_ordinal


def test_to_ordinal_st():
    assert to_ordinal(1) == "1st"
    assert to_ordinal(21) == "21st"
    assert to_ordinal(31) == "31st"
    assert to_ordinal(101) == "101st"
    assert to_ordinal(121) == "121st"


def test_to_ordinal_nd():
    assert to_ordinal(2) == "2nd"
    assert to_ordinal(22) == "22nd"
    assert to_ordinal(32) == "32nd"
    assert to_ordinal(102) == "102nd"
    assert to_ordinal(122) == "122nd"


def test_to_ordinal_rd():
    assert to_ordinal(3) == "3rd"
    assert to_ordinal(23) == "23rd"
    assert to_ordinal(33) == "33rd"
    assert to_ordinal(103) == "103rd"
    assert to_ordinal(123) == "123rd"


def test_to_ordinal_th_special():
    assert to_ordinal(11) == "11th"
    assert to_ordinal(12) == "12th"
    assert to_ordinal(13) == "13th"


def test_to_ordinal_th_special_hundreds():
    assert to_ordinal(111) == "111th"
    assert to_ordinal(112) == "112th"
    assert to_ordinal(113) == "113th"


def test_to_ordinal_th_others():
    assert to_ordinal(4) == "4th"
    assert to_ordinal(5) == "5th"
    assert to_ordinal(6) == "6th"
    assert to_ordinal(7) == "7th"
    assert to_ordinal(8) == "8th"
    assert to_ordinal(9) == "9th"
    assert to_ordinal(10) == "10th"
    assert to_ordinal(20) == "20th"
    assert to_ordinal(100) == "100th"


def test_to_ordinal_zero():
    assert to_ordinal(0) == "0th"


def test_to_ordinal_negative_numbers_use_truncated_remainder():
    assert to_ordinal(-1) == "-1th"
    assert to_ordinal(-7) == "-7th"
    assert to_ordinal(-8) == "-8th"
    assert to_ordinal(-9) == "-9th"
    assert to_ordinal(-11) == "-11th"
    assert to_ordinal(-99) == "-99th"


def test_to_ordinal_integral_float_drops_fraction():
    assert to_ordinal(1.0) == "1st"
    assert to_ordinal(22.0) == "22nd"
    assert to_ordinal(3.0) == "3rd"
    assert to_ordinal(112.0) == "112th"
    assert to_ordinal(4.0) == "4th"


def test_to_ordinal_fractional_float_keeps_fraction():
    assert to_ordinal(1.5) == "1.5th"


def test_to_ordinal_non_finite_values_use_javascript_names():
    assert to_ordinal(math.nan) == "NaNth"
    assert to_ordinal(math.inf) == "Infinityth"
    assert to_ordinal(-math.inf) == "-Infinityth"


def test_to_ordinal_floats_use_javascript_number_text():
    assert to_ordinal(-0.0) == "0th"
    assert to_ordinal(1e21) == "1e+21th"
    assert to_ordinal(1e-7) == "1e-7th"
    assert to_ordinal(2.0000001) == "2.0000001th"
    assert to_ordinal(123456789012345680000.0) == "123456789012345680000th"
