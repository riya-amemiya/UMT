import unittest

from src.math import xoshiro256


class TestXoshiro256(unittest.TestCase):
    def test_basic_generation(self):
        state = [1, 2, 3, 4]
        result = xoshiro256(state)
        self.assertGreaterEqual(result, 0)
        self.assertLess(result, 1)

    def test_custom_range(self):
        state = [1, 2, 3, 4]
        result = xoshiro256(state, 10, 20)
        self.assertGreaterEqual(result, 10)
        self.assertLess(result, 20)

    def test_state_mutation(self):
        state = [1, 2, 3, 4]
        original_state = state.copy()
        xoshiro256(state)

        self.assertNotEqual(state, original_state)

    def test_reproducibility(self):
        state1 = [1, 2, 3, 4]
        state2 = [1, 2, 3, 4]
        result1 = xoshiro256(state1)
        result2 = xoshiro256(state2)
        self.assertEqual(result1, result2)

    def test_sequential_calls(self):
        state = [1, 2, 3, 4]
        results = [xoshiro256(state) for _ in range(10)]

        self.assertEqual(len(set(results)), len(results))

    def test_docstring_example(self):
        state = [1, 2, 3, 4]
        result = xoshiro256(state)
        self.assertTrue(0 <= result < 1)


if __name__ == "__main__":
    unittest.main()
