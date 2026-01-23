import unittest

from src.tool import pipe


class TestPipe(unittest.TestCase):
    def test_basic_pipe(self):
        result = pipe(1).end()
        self.assertEqual(result, 1)

    def test_map(self):
        result = pipe(1).map(lambda x: x + 1).end()
        self.assertEqual(result, 2)

    def test_map_chaining(self):
        result = pipe(1).map(lambda x: x + 1).map(lambda x: x * 2).end()
        self.assertEqual(result, 4)

    def test_when_condition_true(self):
        result = pipe(5).when(lambda x: x > 3, lambda x: x * 2).end()
        self.assertEqual(result, 10)

    def test_when_condition_false(self):
        result = pipe(2).when(lambda x: x > 3, lambda x: x * 2).end()
        self.assertEqual(result, 2)

    def test_tap(self):
        side_effect_value = []
        result = pipe(5).tap(lambda x: side_effect_value.append(x)).end()
        self.assertEqual(result, 5)
        self.assertEqual(side_effect_value, [5])

    def test_filter_strict_pass(self):
        result = pipe(5).filter_strict(lambda x: x > 3).end()
        self.assertEqual(result, 5)

    def test_filter_strict_fail(self):
        with self.assertRaises(ValueError):
            pipe(2).filter_strict(lambda x: x > 3).end()

    def test_filter_with_default_pass(self):
        result = pipe(5).filter_with_default(lambda x: x > 3, 0).end()
        self.assertEqual(result, 5)

    def test_filter_with_default_fail(self):
        result = pipe(2).filter_with_default(lambda x: x > 3, 0).end()
        self.assertEqual(result, 0)

    def test_filter_result_pass(self):
        result = pipe(5).filter_result(lambda x: x > 3).end()
        self.assertEqual(result.type, "success")
        self.assertEqual(result.value, 5)

    def test_filter_result_fail(self):
        result = pipe(2).filter_result(lambda x: x > 3).end()
        self.assertEqual(result.type, "error")
        self.assertIsInstance(result.error, ValueError)

    def test_complex_chain(self):
        result = (
            pipe(1)
            .map(lambda x: x + 1)
            .when(lambda x: x > 1, lambda x: x * 3)
            .tap(lambda x: None)
            .filter_with_default(lambda x: x > 5, 10)
            .end()
        )
        self.assertEqual(result, 6)

    def test_docstring_example(self):
        result = pipe(1).map(lambda x: x + 1).map(lambda x: x * 2).end()
        self.assertEqual(result, 4)


if __name__ == "__main__":
    unittest.main()
