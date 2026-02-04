use umt_rust::ip::{
    ip_to_long, long_to_ip, cidr_to_subnet_mask, subnet_mask_to_cidr,
    ip_to_binary_string, is_in_range, is_private_ip,
    get_ip_class
};

#[test]
fn test_ip_round_trip() {
    let test_ips = vec![
        "192.168.1.1",
        "10.0.0.1",
        "172.16.0.1",
        "8.8.8.8",
        "255.255.255.255",
        "0.0.0.0",
    ];

    for ip in test_ips {
        let long_val = ip_to_long(ip).unwrap();
        let converted = long_to_ip(long_val);
        assert_eq!(converted, ip);
    }
}

#[test]
fn test_cidr_round_trip() {
    let cidr_values = vec![8, 16, 24, 30, 32];
    for cidr in cidr_values {
        let mask = cidr_to_subnet_mask(cidr).unwrap();
        let converted = subnet_mask_to_cidr(&mask).unwrap();
        assert_eq!(converted, cidr);
    }
}

#[test]
fn test_network_range() {
    let ip = "192.168.1.100";
    let network = "192.168.1.0";
    let cidr = 24;

    assert!(is_in_range(ip, network, cidr).unwrap());

    let binary = ip_to_binary_string(ip).unwrap();
    assert_eq!(binary.len(), 32);
}

#[test]
fn test_private_ip() {
    assert!(is_private_ip("192.168.1.1").unwrap());
    assert!(is_private_ip("10.0.0.1").unwrap());
    assert!(!is_private_ip("8.8.8.8").unwrap());

    assert_eq!(get_ip_class("192.168.1.1"), "C");
    assert_eq!(get_ip_class("10.0.0.1"), "A");
}
