import unittest
from src import (
    delete_spaces,
    format_string,
    from_base64,
    to_base64,
    has_no_letters,
    pad_end,
    pad_start,
    random_string,
    DEFAULT_RANDOM_STRING_CHARS,
    random_string_initialization,
    reverse_string,
    to_half_width,
    trim_characters,
    trim_end_characters,
    trim_start_characters,
)


class TestDeleteSpaces(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(delete_spaces("Hello World"), "HelloWorld")
        self.assertEqual(
            delete_spaces("  leading and trailing spaces  "), "leadingandtrailingspaces"
        )
        self.assertEqual(delete_spaces("no_spaces"), "no_spaces")

    def test_edge_cases(self):
        self.assertEqual(delete_spaces(""), "")
        self.assertEqual(delete_spaces("   "), "")
        self.assertEqual(delete_spaces("\t\n\r\f\v"), "")  # All standard whitespace

    def test_docstring_example(self):
        self.assertEqual(delete_spaces("Hello World"), "HelloWorld")
        self.assertEqual(delete_spaces("  tab\t space "), "tabspace")


class TestFormatString(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(format_string("Hello, {0}!", "World"), "Hello, World!")
        self.assertEqual(
            format_string("My name is {0} and I am {1} years old.", "Alice", 30),
            "My name is Alice and I am 30 years old.",
        )
        self.assertEqual(format_string("{0}{1}{0}", "A", "B"), "ABA")

    def test_edge_cases(self):
        self.assertEqual(
            format_string("No placeholders here.", "value"), "No placeholders here."
        )
        self.assertEqual(
            format_string("Empty values: {0}{1}", "", ""), "Empty values: "
        )
        self.assertEqual(
            format_string("Not enough values: {0} {1}", "val1"),
            "Not enough values: val1 {1}",
        )  # Python's format behavior
        self.assertEqual(
            format_string("Too many values: {0}", "val1", "val2"),
            "Too many values: val1",
        )

    def test_docstring_example(self):
        self.assertEqual(format_string("Hello, {0}!", "World"), "Hello, World!")
        self.assertEqual(
            format_string("The sum of {0} and {1} is {2}.", 1, 2, 3),
            "The sum of 1 and 2 is 3.",
        )

    def test_different_types(self):
        self.assertEqual(
            format_string("Number: {0}, Boolean: {1}", 123, True),
            "Number: 123, Boolean: True",
        )


class TestToBase64(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(to_base64("Hello World"), "SGVsbG8gV29ybGQ=")
        self.assertEqual(to_base64("Python"), "UHl0aG9u")
        self.assertEqual(to_base64("12345"), "MTIzNDU=")

    def test_edge_cases(self):
        self.assertEqual(to_base64(""), "")

    def test_special_characters(self):
        self.assertEqual(to_base64("!@#$%^&*()_+"), "IUAjJCVeJiooKV8r")
        self.assertEqual(
            to_base64("Hello World! This is a test."),
            "SGVsbG8gV29ybGQhIFRoaXMgaXMgYSB0ZXN0Lg==",
        )

    def test_unicode_characters(self):
        self.assertEqual(to_base64("こんにちは世界"), "44GT44KT44Gr44Gh44Gv5LiW55WM")
        self.assertEqual(to_base64("😊"), "8J+Yig==")

    def test_docstring_example(self):
        self.assertEqual(to_base64("Hello World"), "SGVsbG8gV29ybGQ=")


class TestFromBase64(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(from_base64("SGVsbG8gV29ybGQ="), "Hello World")
        self.assertEqual(from_base64("UHl0aG9u"), "Python")
        self.assertEqual(from_base64("MTIzNDU="), "12345")

    def test_edge_cases(self):
        self.assertEqual(from_base64(""), "")
        with self.assertRaises(ValueError):
            from_base64("Invalid Base64")
        with self.assertRaises(ValueError):
            from_base64("SGVsbG8gV29ybGQ")

    def test_special_characters(self):
        self.assertEqual(from_base64("IUAjJCVeJiooKV8r"), "!@#$%^&*()_+")
        self.assertEqual(
            from_base64("SGVsbG8gV29ybGQhIFRoaXMgaXMgYSB0ZXN0Lg=="),
            "Hello World! This is a test.",
        )

    def test_unicode_characters(self):
        self.assertEqual(from_base64("44GT44KT44Gr44Gh44Gv5LiW55WM"), "こんにちは世界")
        self.assertEqual(from_base64("8J+Yig=="), "😊")

    def test_docstring_example(self):
        self.assertEqual(from_base64("SGVsbG8gV29ybGQ="), "Hello World")


class TestHasNoLetters(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(has_no_letters("12345"))
        self.assertTrue(has_no_letters("!@#$%^"))
        self.assertFalse(has_no_letters("abcde"))
        self.assertFalse(has_no_letters("Hello World"))

    def test_mixed_content(self):
        self.assertFalse(has_no_letters("abc123"))
        self.assertFalse(has_no_letters("123abc"))
        self.assertFalse(has_no_letters("!@#abc"))
        self.assertTrue(has_no_letters("123 !@#"))

    def test_unicode_characters(self):
        self.assertTrue(has_no_letters("🌟123#"))
        self.assertFalse(has_no_letters("你好世界"))
        self.assertFalse(has_no_letters("Привет"))
        self.assertFalse(has_no_letters("مرحبا"))
        self.assertTrue(has_no_letters("😊👍🎉"))

    def test_edge_cases(self):
        self.assertTrue(has_no_letters(""))

    def test_docstring_examples(self):
        self.assertTrue(has_no_letters("123"))
        self.assertTrue(has_no_letters("🌟123#"))
        self.assertFalse(has_no_letters("abc123"))


class TestPadEnd(unittest.TestCase):
    def test_basic_padding(self):
        self.assertEqual(pad_end("abc", 5, "0"), "abc00")
        self.assertEqual(pad_end("hello", 10, "*"), "hello*****")

    def test_pad_string_longer_than_needed(self):
        self.assertEqual(pad_end("abc", 5, "012345"), "abc01")

    def test_target_length_less_than_string_length(self):
        self.assertEqual(pad_end("hello", 3, "*"), "hello")

    def test_empty_pad_string(self):
        self.assertEqual(pad_end("hello", 8, ""), "hello")

    def test_empty_string(self):
        self.assertEqual(pad_end("", 3, "x"), "xxx")

    def test_target_length_is_zero(self):
        self.assertEqual(pad_end("abc", 0, "0"), "abc")  # String length is already > 0

    def test_docstring_examples(self):
        self.assertEqual(pad_end("abc", 5, "0"), "abc00")
        self.assertEqual(pad_end("hello", 8, "123"), "hello123")
        self.assertEqual(pad_end("world", 3, "!"), "world")
        self.assertEqual(pad_end("test", 6, ""), "test")


class TestPadStart(unittest.TestCase):
    def test_basic_padding(self):
        self.assertEqual(pad_start("123", 5, "0"), "00123")
        self.assertEqual(pad_start("abc", 8, "def"), "defdeabc")

    def test_pad_string_longer_than_needed(self):
        self.assertEqual(pad_start("123", 5, "012345"), "01123")

    def test_target_length_less_than_string_length(self):
        self.assertEqual(pad_start("hello", 3, "*"), "hello")

    def test_empty_pad_string_raises_error(self):
        with self.assertRaises(ValueError):
            pad_start("hello", 8, "")

    def test_empty_string(self):
        self.assertEqual(pad_start("", 3, "x"), "xxx")

    def test_target_length_is_zero(self):
        self.assertEqual(pad_start("abc", 0, "0"), "abc")

    def test_docstring_examples(self):
        self.assertEqual(pad_start("123", 5, "0"), "00123")
        self.assertEqual(pad_start("abc", 8, "def"), "defdeabc")
        self.assertEqual(pad_start("world", 3, "!"), "world")


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
        # random.choice will raise IndexError if char_pool is empty
        with self.assertRaises(IndexError):
            random_string(char_pool="")

    def test_size_zero(self):
        s = random_string(size=0)
        self.assertEqual(s, "")

    def test_docstring_examples(self):
        self.assertEqual(len(random_string()), 8)
        self.assertEqual(len(random_string(10)), 10)
        self.assertTrue(all(c in "abc" for c in random_string(5, "abc")))


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
        with self.assertRaises(IndexError):
            generator(5)

    def test_docstring_example(self):
        custom_random = random_string_initialization("xyz")
        r_string = custom_random(3)
        self.assertEqual(len(r_string), 3)
        self.assertTrue(all(c in "xyz" for c in r_string))


class TestReverseString(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(reverse_string("hello"), "olleh")
        self.assertEqual(reverse_string("Python"), "nohtyP")

    def test_palindrome(self):
        self.assertEqual(reverse_string("madam"), "madam")

    def test_empty_string(self):
        self.assertEqual(reverse_string(""), "")

    def test_string_with_spaces(self):
        self.assertEqual(reverse_string("hello world"), "dlrow olleh")

    def test_string_with_numbers_and_symbols(self):
        self.assertEqual(reverse_string("123!@#"), "#@!321")

    def test_unicode_string(self):
        self.assertEqual(reverse_string("こんにちは"), "はちにんこ")

    def test_docstring_examples(self):
        self.assertEqual(reverse_string("Hello"), "olleH")
        self.assertEqual(reverse_string("madam"), "madam")


class TestToHalfWidth(unittest.TestCase):
    def test_full_width_alphanumeric(self):
        self.assertEqual(
            to_half_width("Ｈｅｌｌｏ Ｗｏｒｌｄ １２３"), "Hello World 123"
        )
        self.assertEqual(to_half_width("ＡＢＣｄｅｆ０１２３"), "ABCdef0123")

    def test_string_with_no_full_width_chars(self):
        self.assertEqual(to_half_width("Hello World 123"), "Hello World 123")

    def test_string_with_mixed_chars(self):
        self.assertEqual(
            to_half_width("Ｈello Ｗorld １２３ and abc"), "Hello World 123 and abc"
        )

    def test_full_width_symbols_and_space(self):
        # Note: Current implementation only handles U+FF01 to U+FF5E and U+3000 (full-width space)
        self.assertEqual(
            to_half_width("！＂＃＄％＆＇（）＊＋，－．／"), "!\"#$%&'()*+,-./"
        )  # subset of symbols
        self.assertEqual(to_half_width("　"), " ")  # Full-width space

    def test_empty_string(self):
        self.assertEqual(to_half_width(""), "")

    def test_docstring_examples(self):
        self.assertEqual(
            to_half_width("Ｈｅｌｌｏ Ｗｏｒｌｄ １２３"), "Hello World 123"
        )
        self.assertEqual(to_half_width("ＡＢＣｄｅｆ"), "ABCdef")


class TestTrimStartCharacters(unittest.TestCase):
    def test_basic_trim(self):
        self.assertEqual(trim_start_characters("---hello", "-"), "hello")
        self.assertEqual(trim_start_characters("...world", "."), "world")

    def test_multiple_chars_to_trim(self):
        self.assertEqual(trim_start_characters("-.-.hello", "-."), "hello")

    def test_no_chars_to_trim(self):
        self.assertEqual(trim_start_characters("hello", "-"), "hello")

    def test_string_is_all_trim_chars(self):
        self.assertEqual(trim_start_characters("----", "-"), "")

    def test_empty_string(self):
        self.assertEqual(trim_start_characters("", "-"), "")

    def test_empty_trim_chars(self):
        self.assertEqual(trim_start_characters("hello", ""), "hello")

    def test_trim_chars_not_at_start(self):
        self.assertEqual(trim_start_characters("hello---", "-"), "hello---")

    def test_docstring_examples(self):
        self.assertEqual(trim_start_characters("!!!hello", "!"), "hello")
        self.assertEqual(trim_start_characters("---123", "-"), "123")
        self.assertEqual(trim_start_characters("abc123", "xyz"), "abc123")


class TestTrimEndCharacters(unittest.TestCase):
    def test_basic_trim(self):
        self.assertEqual(trim_end_characters("hello---", "-"), "hello")
        self.assertEqual(trim_end_characters("world...", "."), "world")

    def test_multiple_chars_to_trim(self):
        self.assertEqual(trim_end_characters("hello-.-.", "-."), "hello")

    def test_no_chars_to_trim(self):
        self.assertEqual(trim_end_characters("hello", "-"), "hello")

    def test_string_is_all_trim_chars(self):
        self.assertEqual(trim_end_characters("----", "-"), "")

    def test_empty_string(self):
        self.assertEqual(trim_end_characters("", "-"), "")

    def test_empty_trim_chars(self):
        self.assertEqual(trim_end_characters("hello", ""), "hello")

    def test_trim_chars_not_at_end(self):
        self.assertEqual(trim_end_characters("---hello", "-"), "---hello")

    def test_docstring_examples(self):
        self.assertEqual(trim_end_characters("hello!!!", "!"), "hello")
        self.assertEqual(trim_end_characters("123---", "-"), "123")
        self.assertEqual(trim_end_characters("abc123", "xyz"), "abc123")


class TestTrimCharacters(unittest.TestCase):
    def test_trim_from_both_ends(self):
        self.assertEqual(trim_characters("-.-hello-.-", "-."), "hello")
        self.assertEqual(trim_characters("...world...", "."), "world")

    def test_trim_from_start_only(self):
        self.assertEqual(trim_characters("---hello", "-"), "hello")

    def test_trim_from_end_only(self):
        self.assertEqual(trim_characters("hello---", "-"), "hello")

    def test_no_chars_to_trim(self):
        self.assertEqual(trim_characters("hello", "-"), "hello")

    def test_string_is_all_trim_chars(self):
        self.assertEqual(trim_characters("-----", "-"), "")

    def test_empty_string(self):
        self.assertEqual(trim_characters("", "-"), "")

    def test_empty_trim_chars(self):
        self.assertEqual(trim_characters("hello", ""), "hello")

    def test_interspersed_chars(self):
        self.assertEqual(trim_characters("-h-e-l-l-o-", "-"), "h-e-l-l-o")

    def test_docstring_examples(self):
        self.assertEqual(trim_characters("-.-hello-.-", "-."), "hello")
        self.assertEqual(trim_characters("123abc123", "123"), "abc")


if __name__ == "__main__":
    unittest.main()
