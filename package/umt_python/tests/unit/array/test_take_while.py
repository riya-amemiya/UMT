import unittest

from src.array import take_while


class TestTakeWhile(unittest.TestCase):
    def test_takes_leading_elements(self):
        self.assertEqual(take_while([1, 2, 3, 4, 1], lambda n, i, arr: n < 3), [1, 2])

    def test_returns_copy_when_all_pass(self):
        source = [1, 2, 3]
        result = take_while(source, lambda n, i, arr: True)
        self.assertEqual(result, [1, 2, 3])
        self.assertIsNot(result, source)

    def test_returns_empty_when_first_fails(self):
        self.assertEqual(take_while([5, 1, 2], lambda n, i, arr: n < 3), [])

    def test_empty_input(self):
        self.assertEqual(take_while([], lambda n, i, arr: True), [])

    def test_passes_value_index_and_array(self):
        source = ["a", "b", "c"]
        calls: list[tuple[str, int, list[str]]] = []

        def predicate(value: str, index: int, array: list[str]) -> bool:
            calls.append((value, index, array))
            return index < 1

        take_while(source, predicate)
        self.assertEqual([(v, i) for v, i, _ in calls], [("a", 0), ("b", 1)])
        self.assertTrue(all(array is source for _, _, array in calls))

    def test_does_not_mutate_input(self):
        source = [1, 2, 3]
        take_while(source, lambda n, i, arr: n < 2)
        self.assertEqual(source, [1, 2, 3])


if __name__ == "__main__":
    unittest.main()
