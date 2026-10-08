import unittest

from src.string import slugify


class TestSlugify(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(slugify("Hello World!"), "hello-world")
        self.assertEqual(slugify("This is a Test"), "this-is-a-test")

    def test_unicode(self):
        self.assertEqual(slugify("Café"), "cafe")

    def test_edge_cases(self):
        self.assertEqual(slugify(""), "")
        self.assertEqual(slugify("hello"), "hello")

    def test_special_characters(self):
        self.assertEqual(slugify("hello---world"), "hello-world")
        self.assertEqual(slugify("--hello--"), "hello")
        self.assertEqual(slugify("Special!@#$%Characters"), "special-characters")
        self.assertEqual(slugify("!!!"), "")

    def test_whitespace_and_underscores(self):
        self.assertEqual(slugify("Hello    World"), "hello-world")
        self.assertEqual(slugify("  Hello World  "), "hello-world")
        self.assertEqual(slugify("snake_case_text"), "snake-case-text")
        self.assertEqual(slugify("__foo_bar__"), "foo-bar")
        self.assertEqual(slugify("--hello-world--"), "hello-world")
        self.assertEqual(slugify("hello-world"), "hello-world")

    def test_unicode_and_cjk(self):
        self.assertEqual(slugify("café"), "cafe")
        self.assertEqual(slugify("naïve"), "naive")
        self.assertEqual(slugify("Café & Naïve!"), "cafe-naive")
        self.assertEqual(slugify("Japanese: こんにちは"), "japanese-こんにちは")

    def test_numbers_and_case(self):
        self.assertEqual(slugify("Test 123"), "test-123")
        self.assertEqual(slugify("Version 2.5"), "version-2-5")
        self.assertEqual(slugify("CamelCase"), "camelcase")

    def test_docstring_example(self):
        self.assertEqual(slugify("Hello World!"), "hello-world")
        self.assertEqual(slugify("This is a Test"), "this-is-a-test")
        self.assertEqual(slugify("Café"), "cafe")


if __name__ == "__main__":
    unittest.main()
