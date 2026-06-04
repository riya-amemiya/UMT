import unittest

from src.string import title_case


class TestTitleCase(unittest.TestCase):
    def test_plain_sentence(self):
        self.assertEqual(title_case("hello world"), "Hello World")

    def test_mixed_separators(self):
        self.assertEqual(title_case("a-quick brown_fox"), "A Quick Brown Fox")

    def test_camel_case(self):
        self.assertEqual(title_case("helloWorld"), "Hello World")

    def test_empty_string(self):
        self.assertEqual(title_case(""), "")


if __name__ == "__main__":
    unittest.main()
