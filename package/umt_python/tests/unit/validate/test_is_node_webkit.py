import unittest

from src.validate import is_node_webkit


class TestIsNodeWebkit(unittest.TestCase):
    def test_non_node_webkit_environment(self):
        self.assertFalse(is_node_webkit())

    def test_returns_boolean(self):
        self.assertIsInstance(is_node_webkit(), bool)

    def test_docstring_example(self):
        self.assertFalse(is_node_webkit())


if __name__ == "__main__":
    unittest.main()
