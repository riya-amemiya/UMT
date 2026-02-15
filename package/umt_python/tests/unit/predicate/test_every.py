import unittest

from src.predicate import every


class TestEvery(unittest.TestCase):
    def test_every_all_pass(self):
        is_positive_even = every(
            lambda n: n > 0,
            lambda n: n % 2 == 0,
        )
        self.assertTrue(is_positive_even(4))
        self.assertTrue(is_positive_even(8))

    def test_every_fail(self):
        is_positive_even = every(
            lambda n: n > 0,
            lambda n: n % 2 == 0,
        )
        self.assertFalse(is_positive_even(-2))
        self.assertFalse(is_positive_even(3))

    def test_every_short_circuit(self):
        second_called = False

        def predicate2():
            nonlocal second_called
            second_called = True
            return True

        combined = every(
            lambda: False,
            predicate2,
        )
        combined()
        self.assertFalse(second_called)

    def test_every_single_predicate(self):
        single = every(lambda n: n > 0)
        self.assertTrue(single(1))
        self.assertFalse(single(-1))

    def test_every_no_predicates(self):
        always = every()
        self.assertTrue(always())

    def test_every_multiple_arguments(self):
        both_positive = every(
            lambda a, b: a > 0,
            lambda a, b: b > 0,
        )
        self.assertTrue(both_positive(1, 2))
        self.assertFalse(both_positive(1, -1))
        self.assertFalse(both_positive(-1, 1))


if __name__ == "__main__":
    unittest.main()
