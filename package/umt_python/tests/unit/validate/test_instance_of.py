import unittest

from src.validate import instance_of


class Animal:
    def __init__(self, name: str) -> None:
        self.name = name


class Dog(Animal):
    def bark(self) -> str:
        return "woof"


class TestInstanceOf(unittest.TestCase):
    def test_accepts_instances_of_the_constructor(self):
        validator = instance_of(Animal)
        result = validator(Animal("rex"))
        self.assertTrue(result.validate)
        self.assertEqual(result.message, "")

    def test_accepts_instances_of_subclasses(self):
        validator = instance_of(Animal)
        self.assertTrue(validator(Dog("rex")).validate)

    def test_rejects_instances_of_unrelated_constructors(self):
        validator = instance_of(Dog)
        self.assertFalse(validator(Animal("rex")).validate)  # type: ignore[arg-type]

    def test_rejects_non_objects(self):
        validator = instance_of(Animal)
        self.assertFalse(validator("rex").validate)  # type: ignore[arg-type]
        self.assertFalse(validator(1).validate)  # type: ignore[arg-type]
        self.assertFalse(validator(None).validate)  # type: ignore[arg-type]

    def test_returns_the_configured_message_on_failure(self):
        validator = instance_of(Animal, "must be an Animal")
        self.assertEqual(validator("rex").message, "must be an Animal")  # type: ignore[arg-type]

    def test_carries_the_inspected_value_through_type(self):
        validator = instance_of(Animal)
        rex = Animal("rex")
        self.assertIs(validator(rex).type, rex)

    def test_docstring_example(self):
        validator = instance_of(int)
        self.assertTrue(validator(123).validate)
        self.assertFalse(validator("123").validate)  # type: ignore[arg-type]
        failure = instance_of(int, "must be an int")("x")  # type: ignore[arg-type]
        self.assertEqual(failure.message, "must be an int")


if __name__ == "__main__":
    unittest.main()
