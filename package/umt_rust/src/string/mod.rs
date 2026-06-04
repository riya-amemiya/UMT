//! String manipulation utilities
//!
//! This module provides various string manipulation functions including:
//! - Case conversion (camelCase, kebab-case)
//! - HTML escaping/unescaping
//! - Base64 encoding/decoding
//! - String similarity and fuzzy search
//! - Padding and trimming
//! - And more...

mod camel_case;
mod capitalize;
mod capitalize_word;
mod constant_case;
mod count_occurrences;
mod deburr;
mod dedent;
mod delete_spaces;
mod ensure_prefix;
mod ensure_suffix;
mod escape_html;
pub mod format_string;
mod from_base64;
mod fuzzy_search;
mod has_no_letters;
mod kebab_case;
mod levenshtein_distance;
mod mask;
mod normalize_whitespace;
mod pad_end;
mod pad_start;
mod pascal_case;
mod random_string;
mod random_string_initialization;
mod remove_prefix;
mod remove_suffix;
mod reverse_string;
mod sanitize_string;
mod slugify;
mod snake_case;
mod split_by_length;
mod string_similarity;
mod strip_ansi;
mod strip_tags;
mod swap_case;
mod title_case;
mod to_base64;
mod to_full_width;
mod to_half_width;
mod trim_characters;
mod trim_end_characters;
mod trim_start_characters;
mod truncate;
mod uncapitalize;
mod unescape_html;
mod word_count;
mod words;

pub use camel_case::umt_camel_case;
pub use capitalize::umt_capitalize;
pub use capitalize_word::umt_capitalize_word;
pub use constant_case::umt_constant_case;
pub use count_occurrences::umt_count_occurrences;
pub use deburr::umt_deburr;
pub use dedent::umt_dedent;
pub use delete_spaces::umt_delete_spaces;
pub use ensure_prefix::umt_ensure_prefix;
pub use ensure_suffix::umt_ensure_suffix;
pub use escape_html::umt_escape_html;
pub use from_base64::umt_from_base64;
pub use fuzzy_search::{FuzzyMatch, umt_fuzzy_search, umt_fuzzy_search_default};
pub use has_no_letters::umt_has_no_letters;
pub use kebab_case::umt_kebab_case;
pub use levenshtein_distance::umt_levenshtein_distance;
pub use mask::{MaskOptions, umt_mask};
pub use normalize_whitespace::umt_normalize_whitespace;
pub use pad_end::umt_pad_end;
pub use pad_start::umt_pad_start;
pub use pascal_case::umt_pascal_case;
pub use random_string::{umt_random_string, umt_random_string_default};
pub use random_string_initialization::umt_random_string_initialization;
pub use remove_prefix::umt_remove_prefix;
pub use remove_suffix::umt_remove_suffix;
pub use reverse_string::umt_reverse_string;
pub use sanitize_string::umt_sanitize_string;
pub use slugify::umt_slugify;
pub use snake_case::umt_snake_case;
pub use split_by_length::umt_split_by_length;
pub use string_similarity::umt_string_similarity;
pub use strip_ansi::umt_strip_ansi;
pub use strip_tags::umt_strip_tags;
pub use swap_case::umt_swap_case;
pub use title_case::umt_title_case;
pub use to_base64::umt_to_base64;
pub use to_full_width::umt_to_full_width;
pub use to_half_width::umt_to_half_width;
pub use trim_characters::umt_trim_characters;
pub use trim_end_characters::umt_trim_end_characters;
pub use trim_start_characters::umt_trim_start_characters;
pub use truncate::{umt_truncate, umt_truncate_default};
pub use uncapitalize::umt_uncapitalize;
pub use unescape_html::umt_unescape_html;
pub use word_count::umt_word_count;
pub use words::umt_words;

pub use format_string::{
    DetectModeResult, FormatOptions, detect_mode, umt_format_string, umt_format_string_indexed,
};
