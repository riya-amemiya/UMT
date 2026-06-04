import unittest

from src.string import pascal_case


class TestPascalCase(unittest.TestCase):
    def test_kebab_case(self):
        self.assertEqual(pascal_case("hello-world"), "HelloWorld")

    def test_snake_case(self):
        self.assertEqual(pascal_case("hello_world"), "HelloWorld")

    def test_space_separated(self):
        self.assertEqual(pascal_case("hello world"), "HelloWorld")

    def test_camel_case_to_pascal(self):
        self.assertEqual(pascal_case("helloWorld"), "HelloWorld")

    def test_empty_string(self):
        self.assertEqual(pascal_case(""), "")

    def test_acronyms(self):
        self.assertEqual(pascal_case("XMLHttpRequest"), "XmlHttpRequest")


if __name__ == "__main__":
    unittest.main()
