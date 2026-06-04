import unittest

from src.validate import is_browser


class TestIsBrowser(unittest.TestCase):
    def test_non_browser_environment(self):
        self.assertFalse(is_browser())

    def test_returns_boolean(self):
        self.assertIsInstance(is_browser(), bool)

    def test_docstring_example(self):
        self.assertFalse(is_browser())


if __name__ == "__main__":
    unittest.main()
