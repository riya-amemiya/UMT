import unittest

from src.object import pick_by


class TestPickBy(unittest.TestCase):
    def test_selects_entries_where_predicate_true(self):
        self.assertEqual(
            pick_by({"a": 1, "b": 2, "c": 3}, lambda v, _k: v > 1),
            {"b": 2, "c": 3},
        )

    def test_returns_empty_when_nothing_matches(self):
        self.assertEqual(pick_by({"a": 1}, lambda _v, _k: False), {})

    def test_returns_all_when_everything_matches(self):
        self.assertEqual(
            pick_by({"a": 1, "b": 2}, lambda _v, _k: True),
            {"a": 1, "b": 2},
        )

    def test_passes_key_as_second_argument(self):
        self.assertEqual(
            pick_by({"a": 1, "b": 2}, lambda _v, key: key == "a"),
            {"a": 1},
        )

    def test_returns_empty_for_empty_input(self):
        self.assertEqual(pick_by({}, lambda _v, _k: True), {})

    def test_does_not_mutate_input(self):
        source = {"a": 1, "b": 2}
        pick_by(source, lambda _v, _k: True)
        self.assertEqual(source, {"a": 1, "b": 2})


if __name__ == "__main__":
    unittest.main()
