import math
import pytest
from src.validate import is_double


class TestIsDouble:
    def test_should_return_true_for_valid_doubles(self):
        assert is_double(1.5) is True
        assert is_double(-1.5) is True
        assert is_double(1.23e-4) is True
        assert is_double("1.5") is True
        assert is_double("1.5", False) is False

    def test_should_return_false_for_invalid_doubles(self):
        assert is_double(1) is False
        assert is_double(1, False) is False
        assert is_double("1") is False
        assert is_double("1", False) is False

        # Special float values
        assert is_double(float("nan")) is False
        assert is_double(float("inf")) is False
        assert is_double(float("-inf")) is False

        # Non-numeric types
        assert is_double(None) is False
        assert is_double(True) is False
        assert is_double(False) is False
        assert is_double({}) is False
        assert is_double([]) is False

        # Testing explicit false/None with loose=False
        assert is_double(float("-inf"), False) is False
        assert is_double(None, False) is False
        assert is_double(True, False) is False
        assert is_double(False, False) is False

    def test_should_work_with_loose_mode(self):
        # Additional checks for string inputs
        assert is_double("3.14159") is True
        assert is_double("-0.001") is True
        assert is_double("invalid") is False
        assert is_double("") is False

    def test_type_separation(self):
        # Ensure bools are not treated as numbers (Python specific parity check)
        assert is_double(True) is False
        assert is_double(False) is False

        # Ensure lists/dicts are rejected (divergence from JS implicit coercion, documented)
        assert is_double([1.5]) is False
