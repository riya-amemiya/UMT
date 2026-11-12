import unittest

from src.string import sentence_case


class TestSentenceCase(unittest.TestCase):
    def test_docstring_examples(self):
        self.assertEqual(sentence_case("helloWorld foo-bar"), "Hello world foo bar")
        self.assertEqual(sentence_case("XMLHttpRequest"), "Xml http request")
        self.assertEqual(sentence_case(""), "")

    def test_lower_cases_rest(self):
        self.assertEqual(sentence_case("HELLO WORLD"), "Hello world")
        self.assertEqual(sentence_case("snake_case_value"), "Snake case value")

    def test_single_word(self):
        self.assertEqual(sentence_case("hello"), "Hello")

    def test_no_words(self):
        self.assertEqual(sentence_case("  --  "), "")

    def test_non_ascii_letters(self):
        self.assertEqual(
            sentence_case("\u00c9CLAIR au CHOCOLAT"), "\u00c9clair au chocolat"
        )


if __name__ == "__main__":
    unittest.main()
