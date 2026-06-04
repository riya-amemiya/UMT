import unittest

from src.string import constant_case


class TestConstantCase(unittest.TestCase):
    def test_camel_case(self):
        self.assertEqual(constant_case("helloWorld"), "HELLO_WORLD")

    def test_space_separated(self):
        self.assertEqual(constant_case("Hello World"), "HELLO_WORLD")

    def test_kebab_case(self):
        self.assertEqual(constant_case("foo-bar-baz"), "FOO_BAR_BAZ")

    def test_single_word(self):
        self.assertEqual(constant_case("hello"), "HELLO")

    def test_empty_string(self):
        self.assertEqual(constant_case(""), "")


if __name__ == "__main__":
    unittest.main()
