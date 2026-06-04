import unittest

from src.object import omit_by


class TestOmitBy(unittest.TestCase):
    def test_removes_entries_where_predicate_true(self):
        self.assertEqual(
            omit_by({"a": 1, "b": None, "c": 3}, lambda v, _k: v is None),
            {"a": 1, "c": 3},
        )

    def test_returns_all_when_predicate_always_false(self):
        self.assertEqual(
            omit_by({"a": 1, "b": 2}, lambda _v, _k: False),
            {"a": 1, "b": 2},
        )

    def test_returns_empty_when_predicate_always_true(self):
        self.assertEqual(omit_by({"a": 1, "b": 2}, lambda _v, _k: True), {})

    def test_passes_key_as_second_argument(self):
        self.assertEqual(
            omit_by({"a": 1, "b": 2}, lambda _v, key: key == "a"),
            {"b": 2},
        )

    def test_returns_empty_for_empty_input(self):
        self.assertEqual(omit_by({}, lambda _v, _k: False), {})


if __name__ == "__main__":
    unittest.main()
