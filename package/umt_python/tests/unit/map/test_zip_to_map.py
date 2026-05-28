import unittest

from src.map import zip_to_map


class TestZipToMap(unittest.TestCase):
    def test_pairs_keys_and_values_at_same_index(self):
        result = zip_to_map(["a", "b", "c"], [1, 2, 3])
        self.assertEqual(result, {"a": 1, "b": 2, "c": 3})

    def test_stops_at_shorter_length_when_keys_is_longer(self):
        result = zip_to_map(["a", "b", "c"], [1, 2])
        self.assertEqual(result, {"a": 1, "b": 2})

    def test_stops_at_shorter_length_when_values_is_longer(self):
        result = zip_to_map(["a", "b"], [1, 2, 3])
        self.assertEqual(result, {"a": 1, "b": 2})

    def test_keeps_latest_value_for_duplicate_keys(self):
        result = zip_to_map(["a", "b", "a"], [1, 2, 3])
        self.assertEqual(result["a"], 3)
        self.assertEqual(result["b"], 2)
        self.assertEqual(len(result), 2)

    def test_accepts_tuple_keys(self):
        k1 = (1, "x")
        k2 = (2, "y")
        result = zip_to_map([k1, k2], ["x", "y"])
        self.assertEqual(result[k1], "x")
        self.assertEqual(result[k2], "y")

    def test_returns_empty_dict_when_either_input_is_empty(self):
        self.assertEqual(zip_to_map([], [1, 2]), {})
        self.assertEqual(zip_to_map(["a"], []), {})

    def test_preserves_insertion_order_of_keys(self):
        result = zip_to_map(["c", "a", "b"], [1, 2, 3])
        self.assertEqual(list(result.keys()), ["c", "a", "b"])


if __name__ == "__main__":
    unittest.main()
