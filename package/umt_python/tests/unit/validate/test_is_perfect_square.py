from src.validate import is_perfect_square


def test_is_perfect_square_integers():
    assert is_perfect_square(16) is True
    assert is_perfect_square(25) is True
    assert is_perfect_square(49) is True
    assert is_perfect_square(0) is True
    assert is_perfect_square(1) is True


def test_is_perfect_square_non_perfect():
    assert is_perfect_square(10) is False
    assert is_perfect_square(15) is False
    assert is_perfect_square(20) is False
    assert is_perfect_square(-4) is False
    assert is_perfect_square(-16) is False


def test_is_perfect_square_large_numbers():
    assert is_perfect_square(100_000_000) is True
    assert is_perfect_square(100_000_002) is False


def test_is_perfect_square_floats():
    assert is_perfect_square(4.0) is True
    assert is_perfect_square(25.0) is True
    assert is_perfect_square(2.25) is False
    assert is_perfect_square(4.5) is False
    assert is_perfect_square(-4.0) is False


def test_is_perfect_square_invalid_types():
    assert is_perfect_square(True) is False
    assert is_perfect_square(False) is False
    assert is_perfect_square("16") is False  # type: ignore
