import unittest

from src.object import invert


class TestInvert(unittest.TestCase):
    def test_swaps_keys_and_values(self):
        self.assertEqual(invert({"a": 1, "b": 2}), {1: "a", 2: "b"})

    def test_string_to_string_swap(self):
        self.assertEqual(invert({"x": "X", "y": "Y"}), {"X": "x", "Y": "y"})

    def test_returns_empty_for_empty_input(self):
        self.assertEqual(invert({}), {})

    def test_overwrites_earlier_keys_on_collision(self):
        self.assertEqual(invert({"a": 1, "b": 1}), {1: "b"})

    def test_does_not_mutate_input(self):
        source = {"a": 1, "b": 2}
        invert(source)
        self.assertEqual(source, {"a": 1, "b": 2})


if __name__ == "__main__":
    unittest.main()
