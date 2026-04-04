import unittest

from src.function import once


class TestOnce(unittest.TestCase):
    def test_invokes_function_only_once(self):
        call_count = 0

        def initialize():
            nonlocal call_count
            call_count += 1
            return 42

        once_fn = once(initialize)

        self.assertEqual(once_fn(), 42)
        self.assertEqual(once_fn(), 42)
        self.assertEqual(once_fn(), 42)
        self.assertEqual(call_count, 1)

    def test_returns_first_result_ignoring_new_arguments(self):
        once_fn = once(lambda a, b: a + b)

        self.assertEqual(once_fn(3, 4), 7)
        self.assertEqual(once_fn(10, 20), 7)

    def test_single_argument_returns_first_result(self):
        once_fn = once(lambda x: x.upper())

        self.assertEqual(once_fn("hello"), "HELLO")
        self.assertEqual(once_fn("world"), "HELLO")

    def test_handles_none_return(self):
        call_count = 0

        def side_effect():
            nonlocal call_count
            call_count += 1

        once_fn = once(side_effect)

        once_fn()
        once_fn()
        once_fn()
        self.assertIsNone(once_fn.result)
        self.assertEqual(call_count, 1)

    def test_string_result(self):
        once_fn = once(lambda: "initialized")

        self.assertEqual(once_fn(), "initialized")
        self.assertEqual(once_fn(), "initialized")

    def test_float_result(self):
        call_count = 0

        def compute():
            nonlocal call_count
            call_count += 1
            return 3.14

        once_fn = once(compute)

        self.assertAlmostEqual(once_fn(), 3.14)
        self.assertAlmostEqual(once_fn(), 3.14)
        self.assertEqual(call_count, 1)

    def test_called_attribute(self):
        once_fn = once(lambda: 42)

        self.assertFalse(once_fn.called)
        once_fn()
        self.assertTrue(once_fn.called)

    def test_docstring_examples(self):
        initialized = once(lambda: 42)
        self.assertEqual(initialized(), 42)
        self.assertEqual(initialized(), 42)
        self.assertTrue(initialized.called)


if __name__ == "__main__":
    unittest.main()
