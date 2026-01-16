import unittest

from src.string import kebab_case


class TestKebabCase(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(kebab_case("helloWorld"), "hello-world")
        self.assertEqual(kebab_case("FooBar"), "foo-bar")
        self.assertEqual(kebab_case("foo_bar_baz"), "foo-bar-baz")

    def test_edge_cases(self):
        self.assertEqual(kebab_case(""), "")
        self.assertEqual(kebab_case("hello"), "hello")

    def test_special_characters(self):
        self.assertEqual(kebab_case("hello world"), "hello-world")

    def test_docstring_example(self):
        self.assertEqual(kebab_case("helloWorld"), "hello-world")
        self.assertEqual(kebab_case("FooBar"), "foo-bar")
        self.assertEqual(kebab_case("foo_bar_baz"), "foo-bar-baz")


if __name__ == "__main__":
    unittest.main()
