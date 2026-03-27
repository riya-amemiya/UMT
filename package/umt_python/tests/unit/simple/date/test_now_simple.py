import unittest
from datetime import datetime

from src.simple.date import now_simple


class TestNowSimple(unittest.TestCase):
    """Tests ported from Rust: test_now_simple.rs"""

    def test_returns_datetime(self):
        result = now_simple()
        self.assertIsInstance(result, datetime)

    def test_default_timezone_is_jst(self):
        result = now_simple()
        # UTC offset should be +09:00
        offset = result.utcoffset()
        assert offset is not None
        self.assertEqual(offset.total_seconds(), 9 * 3600)

    def test_utc_timezone(self):
        result = now_simple(0)
        offset = result.utcoffset()
        assert offset is not None
        self.assertEqual(offset.total_seconds(), 0)

    def test_string_input(self):
        result = now_simple("9")
        offset = result.utcoffset()
        assert offset is not None
        self.assertEqual(offset.total_seconds(), 9 * 3600)

    def test_invalid_string_falls_back_to_default(self):
        result = now_simple("abc")
        self.assertIsInstance(result, datetime)
