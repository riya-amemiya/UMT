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
        self.assertEqual(kebab_case("hello@world"), "hello-world")
        self.assertEqual(kebab_case("foo#bar$baz"), "foo-bar-baz")

    def test_pascal_snake_and_space(self):
        self.assertEqual(kebab_case("HelloWorld"), "hello-world")
        self.assertEqual(kebab_case("FooBarBaz"), "foo-bar-baz")
        self.assertEqual(kebab_case("hello_world"), "hello-world")
        self.assertEqual(kebab_case("foo bar baz"), "foo-bar-baz")
        self.assertEqual(kebab_case("hello-world"), "hello-world")
        self.assertEqual(kebab_case("foo-bar-baz"), "foo-bar-baz")

    def test_mixed_separators_and_numbers(self):
        self.assertEqual(kebab_case("helloWorld_test case"), "hello-world-test-case")
        self.assertEqual(kebab_case("fooBar-baz qux"), "foo-bar-baz-qux")
        self.assertEqual(kebab_case("helloWorld2"), "hello-world2")
        self.assertEqual(kebab_case("testCase123"), "test-case123")
        self.assertEqual(kebab_case("HELLO"), "hello")

    def test_edge_dashes_and_acronyms(self):
        self.assertEqual(kebab_case("-hello-world-"), "hello-world")
        self.assertEqual(kebab_case("_foo_bar_"), "foo-bar")
        self.assertEqual(kebab_case("hello---world"), "hello-world")
        self.assertEqual(kebab_case("foo___bar"), "foo-bar")
        self.assertEqual(kebab_case("XMLHttpRequest"), "xml-http-request")
        self.assertEqual(kebab_case("getElementById"), "get-element-by-id")
        self.assertEqual(kebab_case("HTML"), "html")
        self.assertEqual(kebab_case("XMLParser"), "xml-parser")

    def test_docstring_example(self):
        self.assertEqual(kebab_case("helloWorld"), "hello-world")
        self.assertEqual(kebab_case("FooBar"), "foo-bar")
        self.assertEqual(kebab_case("foo_bar_baz"), "foo-bar-baz")


if __name__ == "__main__":
    unittest.main()
