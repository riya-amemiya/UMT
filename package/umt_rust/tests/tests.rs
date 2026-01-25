mod advance {
    mod test_range_advance;
}

mod array {
    mod test_range;
    mod test_ultra_number_sort;
}

mod crypto {
    mod test_decode_base32;
    mod test_decode_base32_to_string;
    mod test_decode_base58;
    mod test_decode_base58_to_string;
    mod test_encode_base32;
    mod test_encode_base58;
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
    mod test_average;
    mod test_deg_to_rad;
    mod test_deviation_value;
    mod test_factorial;
    mod test_factorize;
    mod test_gcd;
    mod test_get_decimal_length;
    mod test_lcm;
    mod test_linear_congruential_generator;
    mod test_math_separator;
    mod test_ncr;
    mod test_npr;
    mod test_round_of;
    mod test_softmax;
    mod test_value_swap;
}
