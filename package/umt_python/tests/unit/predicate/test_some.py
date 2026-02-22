import unittest

from src.predicate import some


class TestSome(unittest.TestCase):
    def test_some_all_pass(self):
        is_zero_or_negative = some(
            lambda n: n == 0,
            lambda n: n < 0,
        )
        self.assertTrue(is_zero_or_negative(0))
        self.assertTrue(is_zero_or_negative(-1))
        self.assertFalse(is_zero_or_negative(1))

    def test_some_fail(self):
        is_zero_or_negative = some(
            lambda n: n == 0,
            lambda n: n < 0,
        )
        self.assertFalse(is_zero_or_negative(5))

    def test_some_short_circuit(self):
        second_called = False

        def predicate2():
            nonlocal second_called
            second_called = True
            return False

        combined = some(
            lambda: True,
            predicate2,
        )
        combined()
        self.assertFalse(second_called)

    def test_some_single_predicate(self):
        single = some(lambda n: n > 0)
        self.assertTrue(single(1))
        self.assertFalse(single(-1))

    def test_some_no_predicates(self):
        never = some()
        self.assertFalse(never())

    def test_some_multiple_arguments(self):
        either_positive = some(
            lambda a, b: a > 0,
            lambda a, b: b > 0,
        )
        self.assertTrue(either_positive(1, -1))
        self.assertTrue(either_positive(-1, 1))
        self.assertFalse(either_positive(-1, -1))


if __name__ == "__main__":
    unittest.main()
