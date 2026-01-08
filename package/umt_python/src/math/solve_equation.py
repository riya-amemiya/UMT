from .addition import addition
from .division import division
from .multiplication import multiplication
from .round_of import round_of
from .subtraction import subtraction


def solve_equation(
    coefficients: list[list[float]],
    constants: list[float],
) -> list[float]:
    """
    Solves a system of linear equations using Gaussian elimination.

    Args:
        coefficients: Matrix of coefficients.
        constants: Vector of constants.

    Returns:
        Solution vector.

    Example:
        >>> solve_equation([[1, 1], [1, 2]], [4, 10])
        [-2.0, 6.0]
    """
    n = len(constants)

    # Make copies to avoid modifying input
    coef = [row[:] for row in coefficients]
    const = constants[:]

    for index in range(n):
        # Find the max element in the column (partial pivoting)
        max_element = abs(coef[index][index])
        max_row = index
        for row in range(index + 1, n):
            if abs(coef[row][index]) > max_element:
                max_element = abs(coef[row][index])
                max_row = row

        # Swap the row with the max element to the top of the matrix
        for col in range(index, n):
            coef[max_row][col], coef[index][col] = coef[index][col], coef[max_row][col]

        const[max_row], const[index] = const[index], const[max_row]

        # Perform elimination on the rows below
        for row in range(index + 1, n):
            factor = division(coef[row][index], coef[index][index])

            const[row] = subtraction(const[row], multiplication(factor, const[index]))

            for col in range(index, n):
                coef[row][col] = subtraction(
                    coef[row][col], multiplication(factor, coef[index][col])
                )

    solution: list[float] = []

    # Back substitute to find the solution
    for row in range(n - 1, -1, -1):
        total = 0.0
        for col in range(row + 1, n):
            total = addition(
                total, multiplication(coef[row][col], solution[n - col - 1])
            )

        solution.append(division(subtraction(const[row], total), coef[row][row]))

    # Return the solution vector with values rounded to 1 decimal place
    return [round_of(value, 1) for value in reversed(solution)]
