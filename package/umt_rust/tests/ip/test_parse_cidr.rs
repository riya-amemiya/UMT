use umt_rust::ip::{ParsedCidr, parse_cidr};

#[test]
fn test_valid_cidr() {
    for (cidr, network_address, prefix_length) in [
        ("192.168.0.0/24", "192.168.0.0", 24),
        ("192.168.1.130/25", "192.168.1.128", 25),
        ("10.1.2.3/8", "10.0.0.0", 8),
        ("172.31.255.255/12", "172.16.0.0", 12),
        ("255.255.255.255/1", "128.0.0.0", 1),
        ("10.0.0.1/0", "0.0.0.0", 0),
        ("10.0.0.1/32", "10.0.0.1", 32),
    ] {
        assert_eq!(
            parse_cidr(cidr),
            Ok(ParsedCidr {
                network_address: network_address.to_string(),
                prefix_length,
            }),
            "{}",
            cidr
        );
    }
}

#[test]
fn test_invalid_prefix() {
    for cidr in [
        "192.168.0.0",
        "192.168.0.0/",
        "192.168.0.0/33",
        "192.168.0.0/99",
        "192.168.0.0/100",
        "192.168.0.0/-1",
        "192.168.0.0/08",
        "192.168.0.0/+8",
        "192.168.0.0/ 8",
        "192.168.0.0/8\n",
        "192.168.0.0/24/8",
        "192.168.0.0/\u{0662}",
    ] {
        assert_eq!(
            parse_cidr(cidr),
            Err("Invalid CIDR notation".to_string()),
            "{:?}",
            cidr
        );
    }
}

#[test]
fn test_invalid_address() {
    assert_eq!(parse_cidr("/24"), Err("IP address is required".to_string()));
    for cidr in ["192.168.0/24", "256.0.0.0/8", "192.168.00.0/24"] {
        assert_eq!(
            parse_cidr(cidr),
            Err("Invalid IP address format".to_string()),
            "{}",
            cidr
        );
    }
}
