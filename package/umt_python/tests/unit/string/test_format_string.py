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
            "Number: 123, Boolean: True",
        )


if __name__ == "__main__":
    unittest.main()
