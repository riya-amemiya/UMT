//! Validate module tests

mod test_instance_of;
mod test_is_array;
mod test_is_browser;
mod test_is_bun;
mod test_is_deep_equal;
mod test_is_dictionary_object;
mod test_is_double;
mod test_is_equal;
mod test_is_node;
mod test_is_node_webkit;
mod test_is_not_empty;
mod test_is_number;
mod test_is_perfect_square;
mod test_is_prime_number;
mod test_is_string;
mod test_is_value_nan;
mod test_never;
mod test_parse_email;
mod test_template_literal;
mod test_types;

mod any {
    mod test_core;
}

mod array {
    mod test_array_of;
    mod test_core;
    mod test_validate_array_mod;
}

mod bigint {
    mod test_validate_bigint_mod;
}

mod boolean {
    mod test_core;
    mod test_validate_boolean_mod;
}

mod core {
    mod test_core;
}

mod date {
    mod test_validate_date_mod;
}

mod file {
    mod test_core;
}

mod function {
    mod test_core;
}

mod map {
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
    mod test_intersection;
    mod test_nullable;
    mod test_omit_keys;
    mod test_optional;
    mod test_partial;
    mod test_pick_keys;
    mod test_required;
    mod test_union;
    mod test_validate_object_mod;
}

mod set {
    mod test_validate_set_mod;
}

mod string {
    mod test_core;
    mod test_email;
    mod test_exact_length;
    mod test_length;
    mod test_max_length;
    mod test_min_length;
    mod test_number_string;
    mod test_one_of;
    mod test_regex_match;
    mod test_uuid;
    mod test_validate_string_mod;
}

mod unknown {
    mod test_core;
}
