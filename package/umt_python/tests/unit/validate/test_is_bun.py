import unittest

from src.validate import is_bun


class TestIsBun(unittest.TestCase):
    def test_basic_cases(self):
        self.assertFalse(is_bun())

    def test_returns_bool(self):
        self.assertIsInstance(is_bun(), bool)


if __name__ == "__main__":
    unittest.main()
