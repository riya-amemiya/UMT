import unittest

from src import random_string_initialization, DEFAULT_RANDOM_STRING_CHARS


class TestRandomStringInitialization(unittest.TestCase):
    def test_returns_callable(self):
        generator = random_string_initialization()
        self.assertTrue(callable(generator))

    def test_callable_generates_string_default_pool(self):
        generator = random_string_initialization()
        s = generator(6)
        self.assertEqual(len(s), 6)
        self.assertTrue(all(c in DEFAULT_RANDOM_STRING_CHARS for c in s))
        self.assertIsInstance(s, str)

    def test_callable_generates_string_custom_pool(self):
        custom_chars = "xyz789"
        generator = random_string_initialization(char_pool=custom_chars)
        s = generator(7)
        self.assertEqual(len(s), 7)
        self.assertTrue(all(c in custom_chars for c in s))

    def test_callable_with_size_zero(self):
        generator = random_string_initialization()
        s = generator(0)
        self.assertEqual(s, "")

    def test_callable_with_empty_pool_raises_error(self):
        generator = random_string_initialization(char_pool="")
        with self.assertRaises(ValueError):
            generator(5)

    def test_docstring_example(self):
        custom_random = random_string_initialization("xyz")
        r_string = custom_random(3)
        self.assertEqual(len(r_string), 3)
        self.assertTrue(all(c in "xyz" for c in r_string))


if __name__ == "__main__":
    unittest.main()
