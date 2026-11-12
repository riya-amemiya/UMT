import unittest

from src.validate import (
    TupleReturnType,
    boolean,
    literal,
    number,
    object_validator,
    optional,
    string,
    tuple_validator,
)


class TestTupleValidator(unittest.TestCase):
    def test_validates_each_element_with_the_validator_at_the_same_index(self):
        validator = tuple_validator([string(), number(), boolean()])
        self.assertEqual(
            validator(["a", 1, True]),
            TupleReturnType(validate=True, message="", type=["a", 1, True]),
        )

    def test_accepts_python_tuples(self):
        validator = tuple_validator([string(), number()])
        self.assertTrue(validator(("a", 1)).validate)

    def test_propagates_the_message_of_the_first_failing_element(self):
        validator = tuple_validator(
            [
                string([], "first must be a string"),
                number([], "second must be a number"),
            ]
        )
        self.assertEqual(validator(["a", "b"]).message, "second must be a number")
        self.assertEqual(validator([1, "b"]).message, "first must be a string")

    def test_rejects_sequences_whose_length_differs(self):
        validator = tuple_validator([string(), number()], "must be [string, number]")
        for value in (["a"], ["a", 1, 2]):
            result = validator(value)
            self.assertFalse(result.validate)
            self.assertEqual(result.message, "must be [string, number]")

    def test_rejects_non_sequence_values(self):
        validator = tuple_validator([string()], "not a tuple")
        for value in (None, {"0": "a"}, "a", {"a"}):
            result = validator(value)
            self.assertFalse(result.validate)
            self.assertEqual(result.message, "not a tuple")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(tuple_validator([string()])(["a", "b"]).message, "")

    def test_accepts_only_the_empty_sequence_for_an_empty_tuple(self):
        validator = tuple_validator([])
        self.assertTrue(validator([]).validate)
        self.assertFalse(validator([1]).validate)

    def test_checks_the_length_before_optional_element_validators(self):
        validator = tuple_validator([string(), optional(number())])
        self.assertTrue(validator(["a", None]).validate)
        self.assertFalse(validator(["a"]).validate)

    def test_composes_inside_object_validator(self):
        validator = object_validator(
            {
                "point": tuple_validator([number(), number()]),
                "label": tuple_validator([literal("x"), string()]),
            }
        )
        self.assertTrue(validator({"point": [1, 2], "label": ["x", "y"]}).validate)
        self.assertFalse(validator({"point": [1], "label": ["x", "y"]}).validate)


if __name__ == "__main__":
    unittest.main()
