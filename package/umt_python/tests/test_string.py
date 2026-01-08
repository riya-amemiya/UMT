import unittest

from src.string import (
    camel_case,
    escape_html,
    fuzzy_search,
    kebab_case,
    levenshtein_distance,
    slugify,
    string_similarity,
    truncate,
    unescape_html,
)


class TestCamelCase(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(camel_case("hello world"), "helloWorld")
        self.assertEqual(camel_case("foo-bar-baz"), "fooBarBaz")
        self.assertEqual(camel_case("FooBar"), "fooBar")

    def test_edge_cases(self):
        self.assertEqual(camel_case(""), "")
        self.assertEqual(camel_case("hello"), "hello")
        self.assertEqual(camel_case("HELLO"), "hELLO")

    def test_special_characters(self):
        self.assertEqual(camel_case("foo_bar_baz"), "fooBarBaz")
        self.assertEqual(camel_case("foo--bar"), "fooBar")

    def test_docstring_example(self):
        self.assertEqual(camel_case("hello world"), "helloWorld")
        self.assertEqual(camel_case("foo-bar-baz"), "fooBarBaz")
        self.assertEqual(camel_case("FooBar"), "fooBar")


class TestKebabCase(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(kebab_case("helloWorld"), "hello-world")
        self.assertEqual(kebab_case("FooBar"), "foo-bar")
        self.assertEqual(kebab_case("foo_bar_baz"), "foo-bar-baz")

    def test_edge_cases(self):
        self.assertEqual(kebab_case(""), "")
        self.assertEqual(kebab_case("hello"), "hello")

    def test_special_characters(self):
        self.assertEqual(kebab_case("hello world"), "hello-world")

    def test_docstring_example(self):
        self.assertEqual(kebab_case("helloWorld"), "hello-world")
        self.assertEqual(kebab_case("FooBar"), "foo-bar")
        self.assertEqual(kebab_case("foo_bar_baz"), "foo-bar-baz")


class TestEscapeHtml(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(
            escape_html("<script>alert('XSS')</script>"),
            "&lt;script&gt;alert(&#39;XSS&#39;)&lt;/script&gt;",
        )
        self.assertEqual(escape_html("Tom & Jerry"), "Tom &amp; Jerry")
        self.assertEqual(escape_html('"Hello"'), "&quot;Hello&quot;")

    def test_edge_cases(self):
        self.assertEqual(escape_html(""), "")
        self.assertEqual(escape_html("hello"), "hello")

    def test_docstring_example(self):
        self.assertEqual(
            escape_html("<script>alert('XSS')</script>"),
            "&lt;script&gt;alert(&#39;XSS&#39;)&lt;/script&gt;",
        )
        self.assertEqual(escape_html("Tom & Jerry"), "Tom &amp; Jerry")
        self.assertEqual(escape_html('"Hello"'), "&quot;Hello&quot;")


class TestUnescapeHtml(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(
            unescape_html("&lt;script&gt;alert(&quot;Hello&quot;);&lt;/script&gt;"),
            '<script>alert("Hello");</script>',
        )
        self.assertEqual(unescape_html("Tom &amp; Jerry"), "Tom & Jerry")
        self.assertEqual(unescape_html("5 &lt; 10 &amp;&amp; 10 &gt; 5"), "5 < 10 && 10 > 5")

    def test_edge_cases(self):
        self.assertEqual(unescape_html(""), "")
        self.assertEqual(unescape_html("hello"), "hello")

    def test_docstring_example(self):
        self.assertEqual(
            unescape_html("&lt;script&gt;alert(&quot;Hello&quot;);&lt;/script&gt;"),
            '<script>alert("Hello");</script>',
        )
        self.assertEqual(unescape_html("Tom &amp; Jerry"), "Tom & Jerry")


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


class TestTruncate(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(truncate("Hello World", 5), "Hello...")
        self.assertEqual(truncate("Hello World", 5, "~"), "Hello~")
        self.assertEqual(truncate("Hello", 10), "Hello")

    def test_edge_cases(self):
        self.assertEqual(truncate("", 5), "")
        self.assertEqual(truncate("Hello", 5), "Hello")

    def test_invalid_length(self):
        with self.assertRaises(ValueError):
            truncate("Hello", -1)

    def test_docstring_example(self):
        self.assertEqual(truncate("Hello World", 5), "Hello...")
        self.assertEqual(truncate("Hello World", 5, "~"), "Hello~")
        self.assertEqual(truncate("Hello", 10), "Hello")


class TestLevenshteinDistance(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(levenshtein_distance("kitten", "sitting"), 3)
        self.assertEqual(levenshtein_distance("hello", "hello"), 0)
        self.assertEqual(levenshtein_distance("", "abc"), 3)

    def test_edge_cases(self):
        self.assertEqual(levenshtein_distance("", ""), 0)
        self.assertEqual(levenshtein_distance("abc", ""), 3)

    def test_docstring_example(self):
        self.assertEqual(levenshtein_distance("kitten", "sitting"), 3)
        self.assertEqual(levenshtein_distance("hello", "hello"), 0)
        self.assertEqual(levenshtein_distance("", "abc"), 3)


class TestStringSimilarity(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(string_similarity("hello", "hello"), 1.0)
        self.assertAlmostEqual(string_similarity("hello", "hallo"), 0.8)
        self.assertEqual(string_similarity("", "abc"), 0.0)

    def test_edge_cases(self):
        self.assertEqual(string_similarity("", ""), 1.0)
        self.assertEqual(string_similarity("abc", ""), 0.0)

    def test_docstring_example(self):
        self.assertEqual(string_similarity("hello", "hello"), 1.0)
        self.assertAlmostEqual(string_similarity("hello", "hallo"), 0.8)
        self.assertEqual(string_similarity("", "abc"), 0.0)


class TestFuzzySearch(unittest.TestCase):
    def test_basic_cases(self):
        result = fuzzy_search("hello", ["hello", "world", "helo", "help"])
        self.assertEqual(len(result), 3)
        self.assertEqual(result[0]["item"], "hello")
        self.assertEqual(result[0]["score"], 1.0)

    def test_edge_cases(self):
        self.assertEqual(fuzzy_search("", ["hello", "world"]), [])
        result = fuzzy_search("xyz", ["hello", "world"], threshold=0.0)
        self.assertEqual(len(result), 2)

    def test_threshold(self):
        result = fuzzy_search("hello", ["hello", "helo", "help"], threshold=0.9)
        self.assertEqual(len(result), 1)
        self.assertEqual(result[0]["item"], "hello")

    def test_docstring_example(self):
        result = fuzzy_search("hello", ["hello", "world", "helo", "help"])
        self.assertEqual(result[0]["item"], "hello")
        self.assertEqual(result[0]["score"], 1.0)


if __name__ == "__main__":
    unittest.main()
