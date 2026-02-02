//! Integration tests for IP address utility functions
//!
//! Tests the interaction between IP conversion functions:
//! - Round-trip conversions (IP <-> Long <-> Binary)
//! - CIDR and subnet mask conversions
//! - Network calculations with validation

use umt_rust::ip::{
    cidr_to_subnet_mask, get_ip_class, get_network_address, ip_to_binary_string, ip_to_long,
    is_in_range, is_private_ip, long_to_ip, subnet_mask_to_cidr,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_perform_roundtrip_conversion_ip_to_long_to_ip() {
        let test_ips = [
            "192.168.1.1",
            "10.0.0.1",
            "172.16.0.1",
            "8.8.8.8",
            "255.255.255.255",
            "0.0.0.0",
        ];

        for original_ip in test_ips {
            let long_value = ip_to_long(original_ip).unwrap();
            let converted_ip = long_to_ip(long_value);

            assert_eq!(converted_ip, original_ip);
        }
    }

    #[test]
    fn should_perform_roundtrip_conversion_cidr_to_subnet_mask_to_cidr() {
        let cidr_values = [8, 16, 24, 30, 32];

        for original_cidr in cidr_values {
            let subnet_mask = cidr_to_subnet_mask(original_cidr).unwrap();
            let converted_cidr = subnet_mask_to_cidr(&subnet_mask).unwrap();

            assert_eq!(
                converted_cidr, original_cidr,
                "CIDR {} round-trip failed",
                original_cidr
            );
        }
    }

    #[test]
    fn should_convert_ip_to_binary_and_validate_network_calculations() {
        let test_cases = [
            ("192.168.1.100", "255.255.255.0", 24u8, "192.168.1.0"),
            ("10.0.15.200", "255.255.0.0", 16u8, "10.0.0.0"),
            ("172.16.5.10", "255.255.255.240", 28u8, "172.16.5.0"),
        ];

        for (ip, subnet, cidr, expected_network) in test_cases {
            let binary_string = ip_to_binary_string(ip).unwrap();
            assert_eq!(
                binary_string.len(),
                32,
                "Binary string should be 32 characters"
            );

            let network_long = get_network_address(ip, subnet).unwrap();
            let _network_from_long = long_to_ip(network_long);

            let converted_subnet = cidr_to_subnet_mask(cidr).unwrap();
            assert_eq!(converted_subnet, subnet);

            assert!(
                is_in_range(ip, expected_network, cidr).unwrap(),
                "{} should be in range {}",
                ip,
                expected_network
            );
        }
    }

    #[test]
    fn should_validate_private_ip_ranges_with_conversions() {
        let ip_range_tests = [
            ("192.168.1.1", true, "C"),
            ("10.0.0.1", true, "A"),
            ("172.16.1.1", true, "B"),
            ("8.8.8.8", false, "A"),
            ("1.1.1.1", false, "A"),
        ];

        for (ip, expected_private, expected_class) in ip_range_tests {
            assert_eq!(
                is_private_ip(ip).unwrap(),
                expected_private,
                "{} isPrivate should be {}",
                ip,
                expected_private
            );
            assert_eq!(
                get_ip_class(ip),
                expected_class,
                "{} class should be {}",
                ip,
                expected_class
            );

            let long_value = ip_to_long(ip).unwrap();
            let converted_ip = long_to_ip(long_value);
            assert_eq!(is_private_ip(&converted_ip).unwrap(), expected_private);
            assert_eq!(get_ip_class(&converted_ip), expected_class);
        }
    }

    #[test]
    fn should_handle_complex_network_range_validations() {
        let network_tests = [
            (
                "192.168.1.0",
                24u8,
                vec![
                    ("192.168.1.1", true),
                    ("192.168.1.255", true),
                    ("192.168.2.1", false),
                    ("192.168.0.255", false),
                ],
            ),
            (
                "10.0.0.0",
                8u8,
                vec![
                    ("10.255.255.255", true),
                    ("11.0.0.1", false),
                    ("10.192.168.1", true),
                ],
            ),
        ];

        for (network, cidr, test_ips) in network_tests {
            let subnet_mask = cidr_to_subnet_mask(cidr).unwrap();
            let converted_cidr = subnet_mask_to_cidr(&subnet_mask).unwrap();
            assert_eq!(converted_cidr, cidr);

            for (ip, in_range) in test_ips {
                assert_eq!(
                    is_in_range(ip, network, cidr).unwrap(),
                    in_range,
                    "{} in_range({}) should be {}",
                    ip,
                    network,
                    in_range
                );

                let network_long = get_network_address(network, &subnet_mask).unwrap();
                let network_ip_from_long = long_to_ip(network_long);
                assert_eq!(
                    is_in_range(ip, &network_ip_from_long, cidr).unwrap(),
                    in_range
                );
            }
        }
    }

    #[test]
    fn should_perform_binary_string_conversions_with_network_analysis() {
        let test_cases = [
            ("192.168.1.1", "11000000101010000000000100000001"),
            ("255.255.255.255", "11111111111111111111111111111111"),
            ("0.0.0.0", "00000000000000000000000000000000"),
        ];

        for (ip, expected_binary) in test_cases {
            let binary_string = ip_to_binary_string(ip).unwrap();
            assert_eq!(binary_string, expected_binary);

            let long_value = ip_to_long(ip).unwrap();
            let binary_from_long = format!("{:032b}", long_value);
            assert_eq!(binary_from_long, expected_binary);

            let ip_from_long = long_to_ip(long_value);
            assert_eq!(ip_from_long, ip);
        }
    }

    #[test]
    fn should_handle_subnet_calculations_with_multiple_conversion_methods() {
        let subnets = [
            ("192.168.1.0", 26u8, "255.255.255.192", 62u32),
            ("10.0.0.0", 12u8, "255.240.0.0", 1_048_574u32),
        ];

        for (network, cidr, expected_mask, _expected_hosts) in subnets {
            let mask = cidr_to_subnet_mask(cidr).unwrap();
            assert_eq!(mask, expected_mask);

            let converted_cidr = subnet_mask_to_cidr(&mask).unwrap();
            assert_eq!(converted_cidr, cidr);

            let network_long = get_network_address(network, &mask).unwrap();
            let network_from_long = long_to_ip(network_long);

            assert!(is_in_range(network, &network_from_long, cidr).unwrap());

            let binary_network = ip_to_binary_string(&network_from_long).unwrap();
            let binary_mask = ip_to_binary_string(&mask).unwrap();

            assert_eq!(binary_network.len(), 32);
            assert_eq!(binary_mask.len(), 32);

            let mask_ones = binary_mask.chars().filter(|c| *c == '1').count();
            assert_eq!(mask_ones, cidr as usize);
        }
    }

    #[test]
    fn should_validate_edge_cases_in_ip_conversions() {
        let edge_cases = [
            ("127.0.0.1", "A", false),
            ("169.254.1.1", "B", false),
            ("192.168.0.1", "C", true),
        ];

        for (ip, expected_class, is_private) in edge_cases {
            let long_value = ip_to_long(ip).unwrap();
            let binary = ip_to_binary_string(ip).unwrap();
            let reconstructed = long_to_ip(long_value);

            assert_eq!(reconstructed, ip);
            assert_eq!(get_ip_class(&reconstructed), expected_class);
            assert_eq!(is_private_ip(&reconstructed).unwrap(), is_private);

            assert_eq!(binary.len(), 32);
            assert_eq!(u32::from_str_radix(&binary, 2).unwrap(), long_value);
        }
    }
}
