pub mod camel_case;
pub use camel_case::*;

pub mod delete_spaces;
pub use delete_spaces::*;

pub mod escape_html;
pub use escape_html::*;

pub mod from_base64;
pub use from_base64::*;

pub mod fuzzy_search;
pub use fuzzy_search::*;

pub mod has_no_letters;
pub use has_no_letters::*;

pub mod kebab_case;
pub use kebab_case::*;

pub mod levenshtein_distance;
pub use levenshtein_distance::*;

pub mod pad_end;
pub use pad_end::*;

pub mod pad_start;
pub use pad_start::*;

pub mod random_string;
pub use random_string::*;

pub mod reverse_string;
pub use reverse_string::*;

pub mod slugify;
pub use slugify::*;

pub mod string_similarity;
pub use string_similarity::*;

pub mod to_base64;
pub use to_base64::*;

pub mod to_half_width;
pub use to_half_width::*;

pub mod trim_characters;
pub use trim_characters::*;

pub mod trim_end_characters;
pub use trim_end_characters::*;

pub mod trim_start_characters;
pub use trim_start_characters::*;

pub mod truncate;
pub use truncate::*;

pub mod unescape_html;
pub use unescape_html::*;

pub mod format_string;
pub use format_string::{umt_format_string, umt_format_string_indexed, FormatOptions};
