import unittest

from src.predicate.not_ import not_


class TestNot(unittest.TestCase):
    def test_negates_a_truthy_predicate(self) -> None:
        def is_even(n: int) -> bool:
            return n % 2 == 0

        is_odd = not_(is_even)

        self.assertTrue(is_odd(1))
        self.assertFalse(is_odd(2))
        self.assertTrue(is_odd(3))
        self.assertFalse(is_odd(4))

    def test_negates_a_string_predicate(self) -> None:
        def is_empty(s: str) -> bool:
            return len(s) == 0

        is_not_empty = not_(is_empty)

        self.assertFalse(is_not_empty(""))
        self.assertTrue(is_not_empty("hello"))

    def test_handles_predicates_with_multiple_arguments(self) -> None:
        def greater_than(a: int, b: int) -> bool:
            return a > b

        not_greater_than = not_(greater_than)

        self.assertFalse(not_greater_than(5, 3))
        self.assertTrue(not_greater_than(3, 5))
        self.assertTrue(not_greater_than(3, 3))

    def test_passes_through_arguments_correctly(self) -> None:
        received_args: list[str] = []

        def spy(*args: str) -> bool:
            received_args.extend(args)
            return True

        negated = not_(spy)
        negated("a", "b", "c")
        self.assertEqual(received_args, ["a", "b", "c"])

    def test_double_negation_returns_original_result(self) -> None:
        def is_positive(n: int) -> bool:
            return n > 0

        double_not = not_(not_(is_positive))

        self.assertTrue(double_not(5))
        self.assertFalse(double_not(-1))
