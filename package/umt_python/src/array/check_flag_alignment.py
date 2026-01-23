from typing import Protocol, TypeVar


class HasFlag(Protocol):
    """Protocol for objects that have a flag attribute."""

    flag: bool


T = TypeVar("T", bound=HasFlag)


def check_flag_alignment(matrix: list[list[T]]) -> bool:
    """
    Check if flags are aligned in any direction (horizontal, vertical, or diagonal).

    Args:
        matrix: Two-dimensional array of cells containing flag attribute.

    Returns:
        True if flags are aligned in any direction, False otherwise.

    Example:
        >>> class Cell:
        ...     def __init__(self, flag: bool):
        ...         self.flag = flag
        >>> matrix = [
        ...     [Cell(True), Cell(False), Cell(True)],
        ...     [Cell(False), Cell(True), Cell(False)],
        ...     [Cell(False), Cell(True), Cell(True)],
        ... ]
        >>> check_flag_alignment(matrix)
        False
        >>> matrix2 = [
        ...     [Cell(True), Cell(True), Cell(True)],
        ...     [Cell(False), Cell(True), Cell(False)],
        ...     [Cell(False), Cell(True), Cell(True)],
        ... ]
        >>> check_flag_alignment(matrix2)
        True
    """
    rows = len(matrix)
    if rows == 0 or len(matrix[0]) == 0:
        return False
    cols = len(matrix[0])

    # Check horizontal alignment
    for row in matrix:
        if all(cell.flag for cell in row):
            return True

    # Check vertical alignment
    for col_index in range(cols):
        if all(row[col_index].flag for row in matrix):
            return True

    # Check diagonal alignment (top-left to bottom-right)
    if all(matrix[i][i].flag for i in range(min(rows, cols))):
        return True

    # Check diagonal alignment (bottom-left to top-right)
    return all(matrix[i][cols - i - 1].flag for i in range(min(rows, cols)))
