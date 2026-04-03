import unittest

from src.function import memoize


class TestMemoize(unittest.TestCase):
    def test_cache_and_return_same_result(self):
        call_count = 0

        def double(n):
            nonlocal call_count
            call_count += 1
            return n * 2

        memoized = memoize(double)

        self.assertEqual(memoized(5), 10)
        self.assertEqual(memoized(5), 10)
        self.assertEqual(call_count, 1)

    def test_compute_for_different_arguments(self):
        call_count = 0

        def double(n):
            nonlocal call_count
            call_count += 1
            return n * 2

        memoized = memoize(double)

        self.assertEqual(memoized(5), 10)
        self.assertEqual(memoized(10), 20)
        self.assertEqual(call_count, 2)

    def test_expose_cache(self):
        memoized = memoize(lambda n: n * 2)

        memoized(5)
        self.assertEqual(len(memoized.cache), 1)
        self.assertEqual(memoized.cache[5], 10)

    def test_evict_oldest_entry_when_max_size_exceeded(self):
        call_count = 0

        def double(n):
            nonlocal call_count
            call_count += 1
            return n * 2

        memoized = memoize(double, max_size=2)

        memoized(1)
        memoized(2)
        memoized(3)

        self.assertEqual(len(memoized.cache), 2)
        self.assertNotIn(1, memoized.cache)
        self.assertIn(2, memoized.cache)
        self.assertIn(3, memoized.cache)

    def test_custom_resolver(self):
        call_count = 0

        def add(a, b):
            nonlocal call_count
            call_count += 1
            return a + b

        memoized = memoize(add, resolver=lambda a, b: f"{a}-{b}")

        self.assertEqual(memoized(1, 2), 3)
        self.assertEqual(memoized(1, 2), 3)
        self.assertEqual(call_count, 1)

        self.assertEqual(memoized(2, 1), 3)
        self.assertEqual(call_count, 2)

    def test_string_keys(self):
        call_count = 0

        def upper(s):
            nonlocal call_count
            call_count += 1
            return s.upper()

        memoized = memoize(upper)

        self.assertEqual(memoized("hello"), "HELLO")
        self.assertEqual(memoized("hello"), "HELLO")
        self.assertEqual(call_count, 1)

    def test_evict_oldest_with_resolver_and_max_size(self):
        call_count = 0

        def add(a, b):
            nonlocal call_count
            call_count += 1
            return a + b

        memoized = memoize(
            add,
            max_size=2,
            resolver=lambda a, b: f"{a}-{b}",
        )

        memoized(1, 2)
        memoized(3, 4)
        memoized(5, 6)

        self.assertEqual(len(memoized.cache), 2)
        self.assertNotIn("1-2", memoized.cache)
        self.assertIn("3-4", memoized.cache)
        self.assertIn("5-6", memoized.cache)

    def test_docstring_examples(self):
        memoized = memoize(lambda n: n * 2)
        self.assertEqual(memoized(5), 10)
        self.assertEqual(memoized(5), 10)
        self.assertEqual(len(memoized.cache), 1)


if __name__ == "__main__":
    unittest.main()
