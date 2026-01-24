/// Trait for types that have a flag property.
pub trait HasFlag {
    fn flag(&self) -> bool;
}

/// Check if flags are aligned in any direction (horizontal, vertical, or diagonal).
///
/// # Arguments
///
/// * `matrix` - Two-dimensional array of cells containing flags
///
/// # Returns
///
/// `true` if flags are aligned in any direction, `false` otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::array::{umt_check_flag_alignment, HasFlag};
///
/// #[derive(Clone)]
/// struct Cell {
///     value: i32,
///     flag: bool,
/// }
///
/// impl HasFlag for Cell {
///     fn flag(&self) -> bool {
///         self.flag
///     }
/// }
///
/// let matrix = vec![
///     vec![Cell { value: 1, flag: true }, Cell { value: 2, flag: true }, Cell { value: 3, flag: true }],
///     vec![Cell { value: 4, flag: false }, Cell { value: 5, flag: false }, Cell { value: 6, flag: false }],
///     vec![Cell { value: 7, flag: false }, Cell { value: 8, flag: false }, Cell { value: 9, flag: false }],
/// ];
///
/// assert!(umt_check_flag_alignment(&matrix)); // First row is all true
/// ```
pub fn umt_check_flag_alignment<T: HasFlag>(matrix: &[Vec<T>]) -> bool {
    let rows = matrix.len();
    if rows == 0 {
        return false;
    }

    let cols = if !matrix[0].is_empty() {
        matrix[0].len()
    } else {
        return false;
    };

    // Check horizontal alignment
    for row in matrix {
        if row.iter().all(|cell| cell.flag()) {
            return true;
        }
    }

    // Check vertical alignment
    for col in 0..cols {
        if matrix.iter().all(|row| {
            row.get(col).map(|cell| cell.flag()).unwrap_or(false)
        }) {
            return true;
        }
    }

    // Check diagonal alignment (top-left to bottom-right)
    // Only works for square matrices
    if rows == cols {
        if matrix.iter().enumerate().all(|(i, row)| {
            row.get(i).map(|cell| cell.flag()).unwrap_or(false)
        }) {
            return true;
        }

        // Check diagonal alignment (bottom-left to top-right)
        if matrix.iter().enumerate().all(|(i, row)| {
            row.get(cols - i - 1).map(|cell| cell.flag()).unwrap_or(false)
        }) {
            return true;
        }
    }

    false
}

/// Simple version that works with a 2D array of booleans directly.
///
/// # Arguments
///
/// * `matrix` - Two-dimensional array of boolean flags
///
/// # Returns
///
/// `true` if flags are aligned in any direction, `false` otherwise
pub fn umt_check_flag_alignment_bool(matrix: &[Vec<bool>]) -> bool {
    let rows = matrix.len();
    if rows == 0 {
        return false;
    }

    let cols = if !matrix[0].is_empty() {
        matrix[0].len()
    } else {
        return false;
    };

    // Check horizontal alignment
    for row in matrix {
        if row.iter().all(|&flag| flag) {
            return true;
        }
    }

    // Check vertical alignment
    for col in 0..cols {
        if matrix.iter().all(|row| {
            row.get(col).copied().unwrap_or(false)
        }) {
            return true;
        }
    }

    // Check diagonal alignment (top-left to bottom-right)
    // Only works for square matrices
    if rows == cols {
        if matrix.iter().enumerate().all(|(i, row)| {
            row.get(i).copied().unwrap_or(false)
        }) {
            return true;
        }

        // Check diagonal alignment (bottom-left to top-right)
        if matrix.iter().enumerate().all(|(i, row)| {
            row.get(cols - i - 1).copied().unwrap_or(false)
        }) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct Cell {
        #[allow(dead_code)]
        value: i32,
        flag: bool,
    }

    impl HasFlag for Cell {
        fn flag(&self) -> bool {
            self.flag
        }
    }

    #[test]
    fn test_horizontal_alignment() {
        let matrix = vec![
            vec![Cell { value: 1, flag: true }, Cell { value: 2, flag: true }, Cell { value: 3, flag: true }],
            vec![Cell { value: 4, flag: false }, Cell { value: 5, flag: false }, Cell { value: 6, flag: false }],
            vec![Cell { value: 7, flag: false }, Cell { value: 8, flag: false }, Cell { value: 9, flag: false }],
        ];
        assert!(umt_check_flag_alignment(&matrix));
    }

    #[test]
    fn test_vertical_alignment() {
        let matrix = vec![
            vec![Cell { value: 1, flag: true }, Cell { value: 2, flag: false }, Cell { value: 3, flag: false }],
            vec![Cell { value: 4, flag: true }, Cell { value: 5, flag: false }, Cell { value: 6, flag: false }],
            vec![Cell { value: 7, flag: true }, Cell { value: 8, flag: false }, Cell { value: 9, flag: false }],
        ];
        assert!(umt_check_flag_alignment(&matrix));
    }

    #[test]
    fn test_diagonal_top_left_to_bottom_right() {
        let matrix = vec![
            vec![Cell { value: 1, flag: true }, Cell { value: 2, flag: false }, Cell { value: 3, flag: false }],
            vec![Cell { value: 4, flag: false }, Cell { value: 5, flag: true }, Cell { value: 6, flag: false }],
            vec![Cell { value: 7, flag: false }, Cell { value: 8, flag: false }, Cell { value: 9, flag: true }],
        ];
        assert!(umt_check_flag_alignment(&matrix));
    }

    #[test]
    fn test_diagonal_top_right_to_bottom_left() {
        let matrix = vec![
            vec![Cell { value: 1, flag: false }, Cell { value: 2, flag: false }, Cell { value: 3, flag: true }],
            vec![Cell { value: 4, flag: false }, Cell { value: 5, flag: true }, Cell { value: 6, flag: false }],
            vec![Cell { value: 7, flag: true }, Cell { value: 8, flag: false }, Cell { value: 9, flag: false }],
        ];
        assert!(umt_check_flag_alignment(&matrix));
    }

    #[test]
    fn test_no_alignment() {
        // Matrix with no horizontal, vertical, or diagonal alignment
        let matrix = vec![
            vec![Cell { value: 1, flag: true }, Cell { value: 2, flag: false }, Cell { value: 3, flag: true }],
            vec![Cell { value: 4, flag: false }, Cell { value: 5, flag: false }, Cell { value: 6, flag: false }],
            vec![Cell { value: 7, flag: true }, Cell { value: 8, flag: false }, Cell { value: 9, flag: false }],
        ];
        assert!(!umt_check_flag_alignment(&matrix));
    }

    #[test]
    fn test_empty_matrix() {
        let matrix: Vec<Vec<Cell>> = vec![];
        assert!(!umt_check_flag_alignment(&matrix));
    }

    #[test]
    fn test_empty_rows() {
        let matrix: Vec<Vec<Cell>> = vec![vec![]];
        assert!(!umt_check_flag_alignment(&matrix));
    }

    #[test]
    fn test_single_cell_true() {
        let matrix = vec![vec![Cell { value: 1, flag: true }]];
        assert!(umt_check_flag_alignment(&matrix));
    }

    #[test]
    fn test_single_cell_false() {
        let matrix = vec![vec![Cell { value: 1, flag: false }]];
        assert!(!umt_check_flag_alignment(&matrix));
    }

    #[test]
    fn test_bool_matrix() {
        let matrix = vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![false, false, false],
        ];
        assert!(umt_check_flag_alignment_bool(&matrix));
    }

    #[test]
    fn test_bool_matrix_no_alignment() {
        // Matrix with no horizontal, vertical, or diagonal alignment
        let matrix = vec![
            vec![true, false, true],
            vec![false, false, false],
            vec![true, false, false],
        ];
        assert!(!umt_check_flag_alignment_bool(&matrix));
    }
}
