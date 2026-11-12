import unittest

from src.array import generate_number_array


class TestGenerateNumberArray(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(generate_number_array(5), [0, 1, 2, 3, 4])
        self.assertEqual(generate_number_array(5, 10, 14), [10, 11, 12, 13, 14])

    def test_edge_cases(self):
        self.assertEqual(generate_number_array(0), [])
        self.assertEqual(generate_number_array(1), [0])

    def test_min_greater_than_max_returns_descending_values(self):
        self.assertEqual(generate_number_array(5, 10, 5), [10, 8.75, 7.5, 6.25, 5])

    def test_random_values(self):
        result = generate_number_array(5, 0, 10, random_values=True)
        self.assertEqual(len(result), 5)
        for val in result:
            self.assertTrue(0 <= val <= 10)

    def test_docstring_example(self):
        self.assertEqual(generate_number_array(5), [0, 1, 2, 3, 4])
        self.assertEqual(generate_number_array(5, 10, 14), [10, 11, 12, 13, 14])

    def test_negative_and_fractional_length(self):
        self.assertEqual(generate_number_array(-5), [])
        self.assertEqual(generate_number_array(3.7), [0, 1, 2])
        self.assertEqual(generate_number_array(2.9, 5, 10), [5, 10])

    def test_length_one_returns_min(self):
        self.assertEqual(generate_number_array(1, 10, 20), [10])
        self.assertEqual(generate_number_array(1, 5, 15, random_values=True), [5])

    def test_min_greater_than_max_descends(self):
        self.assertEqual(generate_number_array(3, 10, 0), [10, 5, 0])
        self.assertEqual(generate_number_array(3, 2.5, 0.5), [2.5, 1.5, 0.5])

    def test_integer_inputs_with_integer_step(self):
        self.assertEqual(generate_number_array(3, -5, 5), [-5, 0, 5])
        self.assertEqual(generate_number_array(5, 10, 10), [10, 10, 10, 10, 10])
        result = generate_number_array(3, -(2**53 - 1), 2**53 - 1)
        self.assertEqual(result, [-(2**53 - 1), 0, 2**53 - 1])
        self.assertTrue(all(isinstance(value, int) for value in result))

    def test_integer_inputs_with_fractional_step(self):
        self.assertEqual(generate_number_array(4, 0, 10), [0, 10 / 3, 20 / 3, 10])
        self.assertEqual(
            generate_number_array(11, 0, 1),
            [0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1],
        )

    def test_decimal_inputs_keep_precision(self):
        self.assertEqual(generate_number_array(3, 1.5, 2.5), [1.5, 2, 2.5])
        self.assertEqual(generate_number_array(5, 0.1, 0.9), [0.1, 0.3, 0.5, 0.7, 0.9])
        self.assertEqual(generate_number_array(2, 5e-324, 1e-323), [5e-324, 1e-323])
        self.assertEqual(
            generate_number_array(5, 0, 0.3),
            [
                0,
                0.07500000000000001,
                0.15000000000000002,
                0.22500000000000003,
                0.30000000000000004,
            ],
        )

    def test_unsafe_integer_inputs_use_decimal_path(self):
        self.assertEqual(generate_number_array(2, 1e15, 2e15), [1e15, 2e15])
        self.assertEqual(generate_number_array(2, 0, 2.0**60), [0, 2.0**60])

    def test_random_integer_values(self):
        self.assertEqual(
            generate_number_array(5, 10, 10, random_values=True), [10, 10, 10, 10, 10]
        )
        result = generate_number_array(100, 1, 100, random_values=True)
        for value in result:
            self.assertTrue(1 <= value <= 100)
            self.assertEqual(value, int(value))

    def test_random_decimal_values(self):
        result = generate_number_array(20, 0.5, 1.5, random_values=True)
        self.assertEqual(len(result), 20)
        for value in result:
            self.assertIn(value, (0.5, 1.5))


if __name__ == "__main__":
    unittest.main()
