import unittest

from src.tool import create_pipeline


class TestCreatePipeline(unittest.TestCase):
    def test_create_pipeline(self):
        pipeline = create_pipeline(1)
        self.assertIsNotNone(pipeline)

    def test_call_without_transformer(self):
        pipeline = create_pipeline(1)
        self.assertEqual(pipeline(), 1)

    def test_call_with_transformer(self):
        pipeline = create_pipeline(1)
        result = pipeline(lambda x: x + 1)()
        self.assertEqual(result, 2)

    def test_chaining(self):
        result = create_pipeline(1)(lambda x: x + 1)(lambda x: x * 2)()
        self.assertEqual(result, 4)

    def test_complex_transformations(self):
        result = create_pipeline(10)(lambda x: x * 2)(lambda x: x - 5)(
            lambda x: x // 3
        )()
        self.assertEqual(result, 5)

    def test_string_transformation(self):
        result = create_pipeline("hello")(lambda x: x.upper())(lambda x: x + " WORLD")()
        self.assertEqual(result, "HELLO WORLD")

    def test_docstring_example(self):
        pipeline = create_pipeline(1)
        self.assertEqual(pipeline(), 1)
        self.assertEqual(pipeline(lambda x: x + 1)(), 2)


if __name__ == "__main__":
    unittest.main()
