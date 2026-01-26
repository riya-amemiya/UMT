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

    def test_docstring_example(self):
        self.assertEqual(slugify("Hello World!"), "hello-world")
        self.assertEqual(slugify("This is a Test"), "this-is-a-test")
        self.assertEqual(slugify("Café"), "cafe")


if __name__ == "__main__":
    unittest.main()
