//! Validate module tests

mod test_is_array;
mod test_is_deep_equal;
mod test_is_dictionary_object;
mod test_is_double;
mod test_is_equal;
mod test_is_not_empty;
mod test_is_number;
mod test_is_perfect_square;
mod test_is_prime_number;
mod test_is_string;
mod test_is_value_nan;
mod test_parse_email;
mod test_types;

mod array {
    mod test_core;
    mod test_validate_array_mod;
}

mod boolean {
    mod test_core;
    mod test_validate_boolean_mod;
}

mod core {
    mod test_core;
}

mod number {
    mod test_core;
    mod test_double;
    mod test_even;
    mod test_max_value;
    mod test_min_value;
    mod test_odd;
    mod test_prime;
    mod test_validate_number_mod;
}

mod object {
    mod test_core;
    mod test_optional;
    mod test_validate_object_mod;
}

mod string {
    mod test_core;
    mod test_email;
    mod test_length;
    mod test_max_length;
    mod test_min_length;
    mod test_number_string;
    mod test_regex_match;
    mod test_uuid;
    mod test_validate_string_mod;
}
