import unittest

from src import DEFAULT_RANDOM_STRING_CHARS, random_string


class TestRandomString(unittest.TestCase):
    def test_default_parameters(self):
        s = random_string()
        self.assertEqual(len(s), 8)
        self.assertTrue(all(c in DEFAULT_RANDOM_STRING_CHARS for c in s))
        self.assertIsInstance(s, str)

    def test_custom_size(self):
        s = random_string(size=10)
        self.assertEqual(len(s), 10)
        self.assertTrue(all(c in DEFAULT_RANDOM_STRING_CHARS for c in s))

    def test_custom_char_pool(self):
        custom_chars = "abc123"
        s = random_string(char_pool=custom_chars)
        self.assertEqual(len(s), 8)
        self.assertTrue(all(c in custom_chars for c in s))

    def test_custom_size_and_char_pool(self):
        custom_chars = "!@#"
        s = random_string(size=5, char_pool=custom_chars)
        self.assertEqual(len(s), 5)
        self.assertTrue(all(c in custom_chars for c in s))

    def test_empty_char_pool(self):
        self.assertEqual(random_string(char_pool=""), "")

    def test_negative_size(self):
        with self.assertRaises(ValueError):
            random_string(size=-1)

    def test_size_zero(self):
        s = random_string(size=0)
        self.assertEqual(s, "")

    def test_docstring_examples(self):
        self.assertEqual(len(random_string()), 8)
        self.assertEqual(len(random_string(10)), 10)
        self.assertTrue(all(c in "abc" for c in random_string(5, "abc")))


class TestRandomStringTypeScriptParity(unittest.TestCase):
    def test_empty_char_pool_returns_empty_string(self):
        self.assertEqual(random_string(5, ""), "")
        self.assertEqual(random_string(0, ""), "")

    def test_negative_size_with_empty_char_pool_returns_empty_string(self):
        self.assertEqual(random_string(-1, ""), "")

    def test_negative_size_raises_error(self):
        with self.assertRaises(ValueError):
            random_string(-1, "abc")

    def test_positive_size_uses_pool(self):
        self.assertEqual(random_string(4, "z"), "zzzz")


if __name__ == "__main__":
    unittest.main()
