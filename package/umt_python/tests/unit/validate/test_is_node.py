import unittest

from src.validate import is_node


class TestIsNode(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_node())

    def test_returns_bool(self):
        self.assertIsInstance(is_node(), bool)


if __name__ == "__main__":
    unittest.main()
