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
mod delete_spaces;
mod escape_html;
mod from_base64;
mod fuzzy_search;
mod has_no_letters;
mod kebab_case;
mod levenshtein_distance;
mod pad_end;
mod pad_start;
mod random_string;
mod reverse_string;
mod slugify;
mod string_similarity;
mod to_base64;
mod to_half_width;
mod trim_characters;
mod trim_end_characters;
mod trim_start_characters;
mod truncate;
mod unescape_html;

pub use camel_case::umt_camel_case;
pub use delete_spaces::umt_delete_spaces;
pub use escape_html::umt_escape_html;
pub use from_base64::umt_from_base64;
pub use fuzzy_search::{FuzzyMatch, umt_fuzzy_search, umt_fuzzy_search_default};
pub use has_no_letters::umt_has_no_letters;
pub use kebab_case::umt_kebab_case;
pub use levenshtein_distance::umt_levenshtein_distance;
pub use pad_end::umt_pad_end;
pub use pad_start::umt_pad_start;
pub use random_string::{umt_random_string, umt_random_string_default};
pub use reverse_string::umt_reverse_string;
pub use slugify::umt_slugify;
pub use string_similarity::umt_string_similarity;
pub use to_base64::umt_to_base64;
pub use to_half_width::umt_to_half_width;
pub use trim_characters::umt_trim_characters;
pub use trim_end_characters::umt_trim_end_characters;
pub use trim_start_characters::umt_trim_start_characters;
pub use truncate::{umt_truncate, umt_truncate_default};
pub use unescape_html::umt_unescape_html;
