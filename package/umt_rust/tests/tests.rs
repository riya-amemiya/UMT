mod advance {
    mod test_range_advance;
}

mod array {
    mod test_range;
    mod test_ultra_number_sort;
}

mod color;
mod consts;
mod date;

mod crypto {
    mod test_decode_base32;
    mod test_decode_base32_to_string;
    mod test_decode_base58;
    mod test_decode_base58_to_string;
    mod test_encode_base32;
    mod test_encode_base58;
}

mod data_structure {
    mod test_priority_queue;
}

mod error {
    mod test_retry;
    mod test_safe_execute;
}

mod function {
    mod test_curry;
}

mod integration;

mod ip {
    mod test_cidr_to_long;
    mod test_cidr_to_subnet_mask;
    mod test_get_ip_class;
    mod test_get_network_address;
    mod test_ip_to_binary_string;
    mod test_ip_to_long;
    mod test_is_in_range;
    mod test_is_private_ip;
    mod test_long_to_ip;
    mod test_subnet_mask_to_cidr;
}

mod math {
    mod test_addition;
    mod test_average;
    mod test_bitwise;
    mod test_correlation_coefficient;
    mod test_deg_to_rad;
    mod test_deviation_value;
    mod test_division;
    mod test_factorial;
    mod test_factorize;
    mod test_flexible_number_conversion;
    mod test_gcd;
    mod test_get_decimal_length;
    mod test_lcm;
    mod test_linear_congruential_generator;
    mod test_math_converter;
    mod test_math_separator;
    mod test_max;
    mod test_median;
    mod test_min;
    mod test_mode;
    mod test_multiples;
    mod test_multiplication;
    mod test_ncr;
    mod test_nhr;
    mod test_npr;
    mod test_percentile;
    mod test_prime_factorization;
    mod test_quotient;
    mod test_rad_to_deg;
    mod test_random;
    mod test_reduce;
    mod test_repeated_trial;
    mod test_round_of;
    mod test_softmax;
    mod test_solve_equation;
    mod test_standard_deviation;
    mod test_subtract;
    mod test_to_base_n;
    mod test_to_celsius;
    mod test_to_kelvin;
    mod test_uuidv7;
    mod test_value_swap;
    mod test_xoshiro256;
}

mod simple {
    mod test_birthday_simple;
    mod test_day_of_week_simple;
    mod test_deviation_value_simple;
    mod test_now_simple;
    mod test_quick_sort_simple;
}

mod string;

mod types {
    mod test_array_type;
    mod test_clock_type;
}

mod time {
    mod test_convert_time;
    mod test_normalize_time_unit;
}

mod tool {
    mod test_create_pipeline;
    mod test_parse_json;
    mod test_pipe;
}

mod ua {
    mod test_extract_browser_from_user_agent;
    mod test_extract_device_from_user_agent;
    mod test_extract_os_from_user_agent;
    mod test_parse_user_agent;
}

mod unit_module {
    mod test_unit_converter;
}

mod object {
    mod test_has;
    mod test_is_empty;
    mod test_key_by;
    mod test_merge;
    mod test_merge_deep;
    mod test_omit;
    mod test_pick;
    mod test_pick_deep;
}

mod validate;
