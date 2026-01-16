import unittest

from src.string import camel_case


class TestCamelCase(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(camel_case("hello world"), "helloWorld")
        self.assertEqual(camel_case("foo-bar-baz"), "fooBarBaz")
        self.assertEqual(camel_case("FooBar"), "fooBar")

    def test_edge_cases(self):
        self.assertEqual(camel_case(""), "")
        self.assertEqual(camel_case("hello"), "hello")
        self.assertEqual(camel_case("HELLO"), "hELLO")

    def test_special_characters(self):
        self.assertEqual(camel_case("foo_bar_baz"), "fooBarBaz")
        self.assertEqual(camel_case("foo--bar"), "fooBar")

    def test_docstring_example(self):
        self.assertEqual(camel_case("hello world"), "helloWorld")
        self.assertEqual(camel_case("foo-bar-baz"), "fooBarBaz")
        self.assertEqual(camel_case("FooBar"), "fooBar")


if __name__ == "__main__":
    unittest.main()
