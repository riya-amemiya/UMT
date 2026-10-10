import unittest

from src.string import camel_case


class TestCamelCase(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(camel_case("hello world"), "helloWorld")
        self.assertEqual(camel_case("foo-bar-baz"), "fooBarBaz")
        self.assertEqual(camel_case("FooBar"), "fooBar")

    def test_kebab_case(self):
        self.assertEqual(camel_case("hello-world"), "helloWorld")
        self.assertEqual(camel_case("foo-bar-baz"), "fooBarBaz")

    def test_snake_case(self):
        self.assertEqual(camel_case("hello_world"), "helloWorld")
        self.assertEqual(camel_case("foo_bar_baz"), "fooBarBaz")

    def test_space_separated(self):
        self.assertEqual(camel_case("hello world"), "helloWorld")
        self.assertEqual(camel_case("foo bar baz"), "fooBarBaz")

    def test_pascal_case(self):
        self.assertEqual(camel_case("HelloWorld"), "helloWorld")
        self.assertEqual(camel_case("FooBarBaz"), "fooBarBaz")

    def test_already_camel_case(self):
        self.assertEqual(camel_case("helloWorld"), "helloWorld")
        self.assertEqual(camel_case("fooBarBaz"), "fooBarBaz")

    def test_mixed_separators(self):
        self.assertEqual(camel_case("hello-world_test case"), "helloWorldTestCase")
        self.assertEqual(camel_case("foo_bar-baz qux"), "fooBarBazQux")

    def test_numbers(self):
        self.assertEqual(camel_case("hello-world-2"), "helloWorld2")
        self.assertEqual(camel_case("test_case_123"), "testCase123")

    def test_edge_cases(self):
        self.assertEqual(camel_case(""), "")
        self.assertEqual(camel_case("hello"), "hello")
        self.assertEqual(camel_case("HELLO"), "hELLO")

    def test_special_characters(self):
        self.assertEqual(camel_case("foo_bar_baz"), "fooBarBaz")
        self.assertEqual(camel_case("foo--bar"), "fooBar")
        self.assertEqual(camel_case("hello@world"), "helloWorld")
        self.assertEqual(camel_case("foo#bar$baz"), "fooBarBaz")

    def test_leading_trailing_separators(self):
        self.assertEqual(camel_case("-hello-world-"), "helloWorld")
        self.assertEqual(camel_case("_foo_bar_"), "fooBar")
        self.assertEqual(camel_case("hello-world!!!"), "helloWorld")
        self.assertEqual(camel_case("  foo  bar  "), "fooBar")

    def test_consecutive_separators(self):
        self.assertEqual(camel_case("hello---world"), "helloWorld")
        self.assertEqual(camel_case("foo___bar"), "fooBar")

    def test_first_letter_only_lowercased(self):
        self.assertEqual(camel_case("XMLHttpRequest"), "xMLHttpRequest")
        self.assertEqual(camel_case("getElementById"), "getElementById")

    def test_docstring_example(self):
        self.assertEqual(camel_case("hello world"), "helloWorld")
        self.assertEqual(camel_case("foo-bar-baz"), "fooBarBaz")
        self.assertEqual(camel_case("FooBar"), "fooBar")


if __name__ == "__main__":
    unittest.main()
