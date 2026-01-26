use umt_rust::array::{umt_check_flag_alignment, umt_check_flag_alignment_bool, HasFlag};

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
fn test_horizontal_alignment() {
    let matrix = vec![
        vec![
            Cell { value: 10, flag: true },
            Cell { value: 20, flag: true },
            Cell { value: 30, flag: true },
        ],
        vec![
            Cell { value: 40, flag: false },
            Cell { value: 50, flag: false },
            Cell { value: 60, flag: false },
        ],
        vec![
            Cell { value: 70, flag: false },
            Cell { value: 80, flag: false },
            Cell { value: 90, flag: false },
        ],
    ];
    assert!(umt_check_flag_alignment(&matrix));
}

#[test]
fn test_vertical_alignment() {
    let matrix = vec![
        vec![
            Cell { value: 10, flag: false },
            Cell { value: 20, flag: true },
            Cell { value: 30, flag: false },
        ],
        vec![
            Cell { value: 40, flag: false },
            Cell { value: 50, flag: true },
            Cell { value: 60, flag: false },
        ],
        vec![
            Cell { value: 70, flag: false },
            Cell { value: 80, flag: true },
            Cell { value: 90, flag: false },
        ],
    ];
    assert!(umt_check_flag_alignment(&matrix));
}

#[test]
fn test_diagonal_top_left_to_bottom_right() {
    let matrix = vec![
        vec![
            Cell { value: 10, flag: true },
            Cell { value: 20, flag: false },
            Cell { value: 30, flag: false },
        ],
        vec![
            Cell { value: 40, flag: false },
            Cell { value: 50, flag: true },
            Cell { value: 60, flag: false },
        ],
        vec![
            Cell { value: 70, flag: false },
            Cell { value: 80, flag: false },
            Cell { value: 90, flag: true },
        ],
    ];
    assert!(umt_check_flag_alignment(&matrix));
}

#[test]
fn test_diagonal_top_right_to_bottom_left() {
    let matrix = vec![
        vec![
            Cell { value: 10, flag: false },
            Cell { value: 20, flag: false },
            Cell { value: 30, flag: true },
        ],
        vec![
            Cell { value: 40, flag: false },
            Cell { value: 50, flag: true },
            Cell { value: 60, flag: false },
        ],
        vec![
            Cell { value: 70, flag: true },
            Cell { value: 80, flag: false },
            Cell { value: 90, flag: false },
        ],
    ];
    assert!(umt_check_flag_alignment(&matrix));
}

#[test]
fn test_no_alignment() {
    let matrix = vec![
        vec![
            Cell { value: 10, flag: false },
            Cell { value: 20, flag: true },
            Cell { value: 30, flag: false },
        ],
        vec![
            Cell { value: 40, flag: false },
            Cell { value: 50, flag: true },
            Cell { value: 60, flag: false },
        ],
        vec![
            Cell { value: 70, flag: false },
            Cell { value: 80, flag: false },
            Cell { value: 90, flag: false },
        ],
    ];
    assert!(!umt_check_flag_alignment(&matrix));
}

#[test]
fn test_bool_matrix_horizontal() {
    let matrix = vec![
        vec![true, true, true],
        vec![false, false, false],
        vec![false, false, false],
    ];
    assert!(umt_check_flag_alignment_bool(&matrix));
}

#[test]
fn test_bool_matrix_vertical() {
    let matrix = vec![
        vec![true, false, false],
        vec![true, false, false],
        vec![true, false, false],
    ];
    assert!(umt_check_flag_alignment_bool(&matrix));
}

#[test]
fn test_bool_matrix_diagonal() {
    let matrix = vec![
        vec![true, false, false],
        vec![false, true, false],
        vec![false, false, true],
    ];
    assert!(umt_check_flag_alignment_bool(&matrix));
}

#[test]
fn test_bool_matrix_no_alignment() {
    let matrix = vec![
        vec![true, false, true],
        vec![false, false, false],
        vec![true, false, false],
    ];
    assert!(!umt_check_flag_alignment_bool(&matrix));
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
