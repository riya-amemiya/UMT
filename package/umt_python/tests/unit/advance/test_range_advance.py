from src.advance.range_advance import range_advance


class TestRangeAdvance:
    def test_returns_array_from_zero_to_start_when_only_start_is_provided(self) -> None:
        """returns an array of numbers from 0 to start when only start is provided"""
        assert range_advance(5) == [0, 1, 2, 3, 4]

    def test_returns_array_from_start_to_end_when_no_conditional(self) -> None:
        """returns an array of numbers from start to end when both start and end are provided"""
        assert range_advance(2, 5) == [2, 3, 4]

    def test_returns_array_satisfying_conditional(self) -> None:
        """returns an array of numbers that satisfy the conditional expression when a conditional expression is provided"""

        def is_even(num: int | float) -> bool:
            return num % 2 == 0

        assert range_advance(0, 10, is_even) == [0, 2, 4, 6, 8]

    def test_returns_empty_array_when_start_greater_than_end(self) -> None:
        """returns an empty array when start is greater than end"""
        assert range_advance(5, 2) == []

    def test_returns_empty_array_when_start_equals_end_and_condition_not_met(
        self,
    ) -> None:
        """returns an empty array when start is equal to end and the conditional expression is not satisfied"""

        def is_odd(num: int | float) -> bool:
            return num % 2 != 0

        assert range_advance(5, 5, is_odd) == []
