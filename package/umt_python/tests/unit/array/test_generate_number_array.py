import unittest

from src.array import generate_number_array


class TestGenerateNumberArray(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(generate_number_array(5), [0, 1, 2, 3, 4])
        self.assertEqual(generate_number_array(5, 10, 14), [10, 11, 12, 13, 14])

    def test_edge_cases(self):
        self.assertEqual(generate_number_array(0), [])
        self.assertEqual(generate_number_array(1), [0])

    def test_invalid_input(self):
        with self.assertRaises(ValueError):
            generate_number_array(5, 10, 5)

    def test_random_values(self):
        result = generate_number_array(5, 0, 10, random_values=True)
        self.assertEqual(len(result), 5)
        for val in result:
            self.assertTrue(0 <= val <= 10)

    def test_docstring_example(self):
        self.assertEqual(generate_number_array(5), [0, 1, 2, 3, 4])
        self.assertEqual(generate_number_array(5, 10, 14), [10, 11, 12, 13, 14])


if __name__ == "__main__":
    unittest.main()
