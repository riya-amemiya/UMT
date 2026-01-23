import unittest

from src.math import solve_equation


class TestSolveEquation(unittest.TestCase):
    def test_two_variables(self):
        result = solve_equation([[1, 1], [1, 2]], [4, 10])
        self.assertEqual(result, [-2.0, 6.0])

    def test_three_variables(self):
        coefficients = [[1, 1, 1], [2, 1, 1], [1, 2, 1]]
        constants = [6, 7, 8]
        result = solve_equation(coefficients, constants)
        self.assertEqual(result, [1.0, 2.0, 3.0])

    def test_single_equation(self):
        result = solve_equation([[2]], [4])
        self.assertEqual(result, [2.0])

    def test_partial_pivoting(self):
        coefficients = [[0.001, 1], [1, 1]]
        constants = [1, 2]
        result = solve_equation(coefficients, constants)
        self.assertAlmostEqual(result[0], 1.0, places=1)
        self.assertAlmostEqual(result[1], 1.0, places=1)

    def test_docstring_example(self):
        self.assertEqual(solve_equation([[1, 1], [1, 2]], [4, 10]), [-2.0, 6.0])


if __name__ == "__main__":
    unittest.main()
