import unittest
from datetime import datetime

from src.date import is_between


class TestIsBetween(unittest.TestCase):
    def setUp(self) -> None:
        self.start = datetime(2025, 4, 10, 10, 0, 0)
        self.mid = datetime(2025, 4, 15, 12, 0, 0)
        self.end = datetime(2025, 4, 20, 18, 0, 0)

    def test_returns_true_for_timestamp_strictly_inside_range(self):
        self.assertTrue(is_between(self.mid, self.start, self.end))

    def test_returns_false_for_start_bound_with_default_exclusivity(self):
        self.assertFalse(is_between(self.start, self.start, self.end))

    def test_returns_false_for_end_bound_with_default_exclusivity(self):
        self.assertFalse(is_between(self.end, self.start, self.end))

    def test_includes_both_bounds_when_inclusivity_is_closed(self):
        self.assertTrue(is_between(self.start, self.start, self.end, None, "[]"))
        self.assertTrue(is_between(self.end, self.start, self.end, None, "[]"))

    def test_includes_start_and_excludes_end_when_half_open(self):
        self.assertTrue(is_between(self.start, self.start, self.end, None, "[)"))
        self.assertFalse(is_between(self.end, self.start, self.end, None, "[)"))

    def test_excludes_start_and_includes_end_when_half_closed(self):
        self.assertFalse(is_between(self.start, self.start, self.end, None, "(]"))
        self.assertTrue(is_between(self.end, self.start, self.end, None, "(]"))

    def test_returns_false_when_start_is_after_end(self):
        self.assertFalse(is_between(self.mid, self.end, self.start))

    def test_returns_true_for_same_day_at_day_granularity(self):
        self.assertTrue(
            is_between(
                datetime(2025, 4, 15, 23, 59),
                datetime(2025, 4, 15, 0, 0),
                datetime(2025, 4, 15, 1, 0),
                "day",
                "[]",
            )
        )

    def test_returns_false_for_adjacent_days_at_day_granularity(self):
        self.assertFalse(
            is_between(
                datetime(2025, 4, 16),
                datetime(2025, 4, 10),
                datetime(2025, 4, 15),
                "day",
                "[]",
            )
        )

    def test_uses_sunday_start_week_boundaries(self):
        self.assertTrue(
            is_between(
                datetime(2025, 4, 19),
                datetime(2025, 4, 13),
                datetime(2025, 4, 19),
                "week",
                "[]",
            )
        )
        self.assertFalse(
            is_between(
                datetime(2025, 4, 20),
                datetime(2025, 4, 13),
                datetime(2025, 4, 19),
                "week",
                "[]",
            )
        )

    def test_uses_quarter_granularity(self):
        self.assertTrue(
            is_between(
                datetime(2025, 6, 30),
                datetime(2025, 4, 1),
                datetime(2025, 6, 1),
                "quarter",
                "[]",
            )
        )
        self.assertFalse(
            is_between(
                datetime(2025, 7, 1),
                datetime(2025, 4, 1),
                datetime(2025, 6, 1),
                "quarter",
                "[]",
            )
        )


if __name__ == "__main__":
    unittest.main()
