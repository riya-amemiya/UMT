import unittest

from src.array import check_flag_alignment


class Cell:
    """Helper class for testing check_flag_alignment."""

    def __init__(self, flag: bool):
        self.flag = flag


class TestCheckFlagAlignment(unittest.TestCase):
    def test_empty_matrix(self):
        """Test with empty matrix."""
        result = check_flag_alignment([])
        self.assertFalse(result)

    def test_empty_rows(self):
        """Test with matrix that has empty rows."""
        result = check_flag_alignment([[]])
        self.assertFalse(result)

    def test_horizontal_alignment(self):
        """Test horizontal alignment detection (row of all True)."""
        matrix = [
            [Cell(True), Cell(True), Cell(True)],
            [Cell(False), Cell(True), Cell(False)],
            [Cell(False), Cell(True), Cell(True)],
        ]
        result = check_flag_alignment(matrix)
        self.assertTrue(result)

    def test_vertical_alignment(self):
        """Test vertical alignment detection (column of all True)."""
        matrix = [
            [Cell(False), Cell(True), Cell(False)],
            [Cell(False), Cell(True), Cell(False)],
            [Cell(False), Cell(True), Cell(False)],
        ]
        result = check_flag_alignment(matrix)
        self.assertTrue(result)

    def test_diagonal_top_left_to_bottom_right(self):
        """Test diagonal alignment (top-left to bottom-right)."""
        matrix = [
            [Cell(True), Cell(False), Cell(False)],
            [Cell(False), Cell(True), Cell(False)],
            [Cell(False), Cell(False), Cell(True)],
        ]
        result = check_flag_alignment(matrix)
        self.assertTrue(result)

    def test_diagonal_bottom_left_to_top_right(self):
        """Test diagonal alignment (bottom-left to top-right)."""
        matrix = [
            [Cell(False), Cell(False), Cell(True)],
            [Cell(False), Cell(True), Cell(False)],
            [Cell(True), Cell(False), Cell(False)],
        ]
        result = check_flag_alignment(matrix)
        self.assertTrue(result)

    def test_no_alignment(self):
        """Test when no alignment exists."""

        matrix = [
            [Cell(False), Cell(True), Cell(False)],
            [Cell(True), Cell(False), Cell(False)],
            [Cell(False), Cell(False), Cell(True)],
        ]
        result = check_flag_alignment(matrix)
        self.assertFalse(result)

    def test_all_false(self):
        """Test matrix with all False flags."""
        matrix = [
            [Cell(False), Cell(False), Cell(False)],
            [Cell(False), Cell(False), Cell(False)],
            [Cell(False), Cell(False), Cell(False)],
        ]
        result = check_flag_alignment(matrix)
        self.assertFalse(result)

    def test_all_true(self):
        """Test matrix with all True flags."""
        matrix = [
            [Cell(True), Cell(True), Cell(True)],
            [Cell(True), Cell(True), Cell(True)],
            [Cell(True), Cell(True), Cell(True)],
        ]
        result = check_flag_alignment(matrix)
        self.assertTrue(result)

    def test_single_element_true(self):
        """Test single element matrix with True flag."""
        matrix = [[Cell(True)]]
        result = check_flag_alignment(matrix)
        self.assertTrue(result)

    def test_single_element_false(self):
        """Test single element matrix with False flag."""
        matrix = [[Cell(False)]]
        result = check_flag_alignment(matrix)
        self.assertFalse(result)

    def test_non_square_matrix(self):
        """Test non-square matrix."""

        matrix = [
            [Cell(False), Cell(True), Cell(False), Cell(True)],
            [Cell(True), Cell(True), Cell(True), Cell(True)],
        ]
        result = check_flag_alignment(matrix)
        self.assertTrue(result)

    def test_non_square_matrix_vertical(self):
        """Test non-square matrix with vertical alignment."""

        matrix = [
            [Cell(True), Cell(False)],
            [Cell(True), Cell(False)],
            [Cell(True), Cell(False)],
            [Cell(True), Cell(False)],
        ]
        result = check_flag_alignment(matrix)
        self.assertTrue(result)

    def test_docstring_examples(self):
        """Test examples from docstring."""

        matrix1 = [
            [Cell(True), Cell(False), Cell(True)],
            [Cell(False), Cell(True), Cell(False)],
            [Cell(False), Cell(True), Cell(True)],
        ]
        self.assertTrue(check_flag_alignment(matrix1))

        matrix2 = [
            [Cell(True), Cell(True), Cell(True)],
            [Cell(False), Cell(True), Cell(False)],
            [Cell(False), Cell(True), Cell(True)],
        ]
        self.assertTrue(check_flag_alignment(matrix2))


if __name__ == "__main__":
    unittest.main()
