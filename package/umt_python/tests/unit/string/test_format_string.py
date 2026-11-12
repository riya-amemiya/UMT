import unittest

from src import format_string


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
            "Number: 123, Boolean: true",
        )


class TestFormatStringIndexedMode(unittest.TestCase):
    def test_javascript_string_conversion(self):
        self.assertEqual(format_string("{0} {1}", True, None), "true {1}")
        self.assertEqual(format_string("{0} {1} {2}", 1.0, 1e21, 0.1), "1 1e+21 0.1")
        self.assertEqual(
            format_string("{0}", [1, [2, 3], None, {"a": 1}]),
            "1,2,3,,[object Object]",
        )

    def test_index_follows_javascript_number_rules(self):
        self.assertEqual(format_string("{ 0 } {} { }", "a"), "a {} a")
        self.assertEqual(
            format_string("{0x1} {1e0} {1.5} {-1} {+1}", "a", "b"),
            "b b {1.5} {-1} b",
        )
        self.assertEqual(format_string("{name}", "a"), "{name}")

    def test_escaped_braces(self):
        self.assertEqual(format_string("{{0}} {0}", "v"), "{0} v")

    def test_no_values(self):
        self.assertEqual(format_string("{0}"), "{0}")

    def test_mapping_with_extra_values_is_indexed(self):
        self.assertEqual(format_string("{0} {1}", {"a": 1}, "x"), "[object Object] x")


class TestFormatStringNamedMode(unittest.TestCase):
    def test_paths(self):
        self.assertEqual(
            format_string(
                "{user.name} {items[0]} {items[-1]} {items.0} {items.length}",
                {"user": {"name": "A"}, "items": ["x", "y"]},
            ),
            "A x y x 2",
        )

    def test_default_values(self):
        self.assertEqual(
            format_string("{user.name|Guest} {missing|Guest}", {"user": {"name": "A"}}),
            "A Guest",
        )
        self.assertEqual(format_string("{a|b|c}", {}), "b")
        self.assertEqual(format_string("{a}", {"a": None}), "{a}")

    def test_falsy_values_are_kept(self):
        self.assertEqual(format_string("{a} {b}", {"a": 0, "b": ""}), "0 ")
        self.assertEqual(format_string("{a}", {"a": False}), "false")

    def test_formatters(self):
        self.assertEqual(format_string("{name:upper}", {"name": "x"}), "X")
        self.assertEqual(format_string("{n:lower}", {"n": "ABC"}), "abc")
        self.assertEqual(format_string("{a:upper|x}", {}), "X")
        self.assertEqual(format_string("{a|x:upper}", {}), "x:upper")
        self.assertEqual(
            format_string("{a:unknown} {a:upper(}", {"a": "x"}),
            "x x",
        )

    def test_path_is_not_trimmed_around_formatter_separator(self):
        self.assertEqual(
            format_string("{name : upper}", {"name": "x"}), "{name : upper}"
        )

    def test_custom_formatters(self):
        options = {
            "formatters": {"wrap": lambda value, left, right: f"{left}{value}{right}"}
        }
        self.assertEqual(
            format_string("{a:wrap([,])} {a:upper}", {"a": "x"}, options),
            "[x] X",
        )

    def test_options_without_formatters_switches_to_indexed_mode(self):
        self.assertEqual(
            format_string("{0} {1}", {"a": 1}, {"b": 2}),
            "[object Object] [object Object]",
        )

    def test_built_in_formatters(self):
        self.assertEqual(format_string("{0:number}", 1234.5), "1,234.5")
        self.assertEqual(format_string("{0:pad(4,ab)}", 5), "aba5")
        self.assertEqual(format_string("{0:plural(item,items)}", " 1 "), "item")


if __name__ == "__main__":
    unittest.main()
