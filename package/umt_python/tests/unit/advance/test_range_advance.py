import unittest

from src.advance.range_advance import range_advance


class TestRangeAdvance(unittest.TestCase):
    def test_returns_an_array_of_numbers_from_0_to_start_when_only_start_is_provided(
        self,
    ):
        self.assertEqual(range_advance(5), [0, 1, 2, 3, 4])

    def test_returns_an_array_of_numbers_from_start_to_end_when_both_start_and_end_are_provided(
        self,
    ):
        self.assertEqual(range_advance(2, 5), [2, 3, 4])

    def test_returns_an_array_of_numbers_that_satisfy_the_conditional_expression_when_a_conditional_expression_is_provided(
        self,
    ):
        def is_even(num: int | float) -> bool:
            return num % 2 == 0

        self.assertEqual(range_advance(0, 10, is_even), [0, 2, 4, 6, 8])

    def test_returns_an_empty_array_when_start_is_greater_than_end(self):
        self.assertEqual(range_advance(5, 2), [])

    def test_returns_an_empty_array_when_start_is_equal_to_end_and_the_conditional_expression_is_not_satisfied(
        self,
    ):
        def is_odd(num: int | float) -> bool:
            return num % 2 != 0

        self.assertEqual(range_advance(5, 5, is_odd), [])


if __name__ == "__main__":
    unittest.main()
