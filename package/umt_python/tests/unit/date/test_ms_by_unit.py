import unittest

from src.date import ms_by_unit


class TestMsByUnit(unittest.TestCase):
    def test_maps_fixed_units_to_milliseconds(self):
        self.assertEqual(ms_by_unit["ms"], 1)
        self.assertEqual(ms_by_unit["s"], 1000)
        self.assertEqual(ms_by_unit["m"], 60_000)
        self.assertEqual(ms_by_unit["h"], 3_600_000)
        self.assertEqual(ms_by_unit["d"], 86_400_000)
        self.assertEqual(ms_by_unit["w"], 604_800_000)

    def test_omits_calendar_aware_units(self):
        self.assertIsNone(ms_by_unit.get("M"))
        self.assertIsNone(ms_by_unit.get("y"))


if __name__ == "__main__":
    unittest.main()
