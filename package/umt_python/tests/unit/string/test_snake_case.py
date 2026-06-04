import unittest

from src.string import snake_case


class TestSnakeCase(unittest.TestCase):
    def test_camel_case(self):
        self.assertEqual(snake_case("helloWorld"), "hello_world")

    def test_space_separated(self):
        self.assertEqual(snake_case("Hello World"), "hello_world")

    def test_kebab_case(self):
        self.assertEqual(snake_case("foo-bar-baz"), "foo_bar_baz")

    def test_pascal_case(self):
        self.assertEqual(snake_case("FooBar"), "foo_bar")

    def test_empty_string(self):
        self.assertEqual(snake_case(""), "")

    def test_acronyms(self):
        self.assertEqual(snake_case("XMLHttpRequest"), "xml_http_request")


if __name__ == "__main__":
    unittest.main()
