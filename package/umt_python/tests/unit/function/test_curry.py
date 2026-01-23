import unittest

from src.function import curry


class TestCurry(unittest.TestCase):
    def test_basic_curry(self):
        def add(a, b, c):
            return a + b + c

        curried_add = curry(add)
        self.assertEqual(curried_add(1)(2)(3), 6)

    def test_partial_application(self):
        def add(a, b, c):
            return a + b + c

        curried_add = curry(add)
        add_one = curried_add(1)
        add_one_two = add_one(2)
        self.assertEqual(add_one_two(3), 6)

    def test_all_args_at_once(self):
        def add(a, b, c):
            return a + b + c

        curried_add = curry(add)
        self.assertEqual(curried_add(1, 2, 3), 6)

    def test_mixed_args(self):
        def add(a, b, c):
            return a + b + c

        curried_add = curry(add)
        self.assertEqual(curried_add(1, 2)(3), 6)
        self.assertEqual(curried_add(1)(2, 3), 6)

    def test_preserves_function_metadata(self):
        def my_function(a, b):
            """My docstring."""
            return a + b

        curried = curry(my_function)
        self.assertEqual(curried.__name__, "my_function")
        self.assertEqual(curried.__doc__, "My docstring.")

    def test_two_arg_function(self):
        def multiply(a, b):
            return a * b

        curried_multiply = curry(multiply)
        self.assertEqual(curried_multiply(3)(4), 12)
        self.assertEqual(curried_multiply(3, 4), 12)

    def test_single_arg_function(self):
        def identity(x):
            return x

        curried_identity = curry(identity)
        self.assertEqual(curried_identity(5), 5)

    def test_with_different_types(self):
        def concat(a, b, c):
            return str(a) + str(b) + str(c)

        curried_concat = curry(concat)
        self.assertEqual(curried_concat("hello")(" ")("world"), "hello world")

    def test_docstring_examples(self):
        def add(a, b, c):
            return a + b + c

        curried_add = curry(add)
        self.assertEqual(curried_add(1)(2)(3), 6)
        self.assertEqual(curried_add(1, 2)(3), 6)
        self.assertEqual(curried_add(1, 2, 3), 6)


if __name__ == "__main__":
    unittest.main()
