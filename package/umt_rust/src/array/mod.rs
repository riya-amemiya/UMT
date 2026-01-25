// Sorting helpers module (internal utilities)
pub mod sorting_helpers;

// Basic array functions
pub mod first;
pub use first::*;

pub mod pop;
pub use pop::*;

pub mod sum;
pub use sum::*;

pub mod unique;
pub use unique::*;

pub mod compact;
pub use compact::*;

// Array manipulation functions
pub mod chunk;
pub use chunk::*;

pub mod drop;
pub use drop::*;

pub mod arrays_join;
pub use arrays_join::*;

// Array set operations
pub mod get_arrays_common;
pub use get_arrays_common::*;

pub mod get_arrays_diff;
pub use get_arrays_diff::*;

pub mod uniq_by;
pub use uniq_by::*;

// Array utility functions
pub mod binary_search;
pub use binary_search::*;

pub mod compare_function_default;
pub use compare_function_default::*;

pub mod generate_number_array;
pub use generate_number_array::*;

pub mod group_by;
pub use group_by::*;

pub mod range;
pub use range::*;

// Shuffle functions
pub mod shuffle;
pub use shuffle::*;

pub mod shuffle_2d_array;
pub use shuffle_2d_array::*;

pub mod random_select;
pub use random_select::*;

// Zip functions
pub mod zip;
pub use zip::*;

pub mod zip_longest;
pub use zip_longest::*;

// Sorting algorithms
pub mod insertion_sort;
pub use insertion_sort::*;

pub mod merge_sort;
pub use merge_sort::*;

pub mod quick_sort;
pub use quick_sort::*;

pub mod dual_pivot_quick_sort;
pub use dual_pivot_quick_sort::*;

pub mod tim_sort;
pub use tim_sort::*;

pub mod ultra_number_sort;
pub use ultra_number_sort::*;

// Check flag alignment
pub mod check_flag_alignment;
pub use check_flag_alignment::*;
