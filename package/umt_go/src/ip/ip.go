package ip

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

// validateIP validates an IPv4 address string and returns the parts.
// It checks for empty string, invalid characters, correct number of octets,
// leading zeros, and valid octet ranges.
func validateIP(ipStr string) ([]string, error) {
	if ipStr == "" {
		return nil, fmt.Errorf("IP address is required")
	}

	// Check for invalid characters
	matched, _ := regexp.MatchString(`[^0-9.]`, ipStr)
	if matched {
		return nil, fmt.Errorf("Invalid IP address format")
	}

	parts := strings.Split(ipStr, ".")
	if len(parts) != 4 {
		return nil, fmt.Errorf("Invalid IP address format")
	}

	for _, octet := range parts {
		// Check for empty octet or leading zeros
		if octet == "" || (len(octet) > 1 && octet[0] == '0') {
			return nil, fmt.Errorf("Invalid IP address format")
		}

		num, err := strconv.Atoi(octet)
		if err != nil || num < 0 || num > 255 {
			return nil, fmt.Errorf("Invalid IP address format")
		}
	}

	return parts, nil
}

// IpToBinaryString converts an IPv4 address to its 32-bit binary string representation.
func IpToBinaryString(ipStr string) (string, error) {
	parts, err := validateIP(ipStr)
	if err != nil {
		return "", err
	}

	var sb strings.Builder
	for _, octet := range parts {
		num, _ := strconv.Atoi(octet)
		sb.WriteString(fmt.Sprintf("%08b", num))
	}
	return sb.String(), nil
}

// IpToLong converts an IPv4 address to a 32-bit unsigned integer (as int64).
func IpToLong(ipStr string) (int64, error) {
	binStr, err := IpToBinaryString(ipStr)
	if err != nil {
		return 0, err
	}
	val, err := strconv.ParseInt(binStr, 2, 64)
	if err != nil {
		return 0, fmt.Errorf("Invalid IP address format")
	}
	return val, nil
}

// LongToIp converts a 32-bit unsigned integer to an IPv4 address string.
func LongToIp(long int64) string {
	if long < 0 || long > 0xFFFFFFFF {
		panic("Input must be a valid 32-bit unsigned integer")
	}

	binary := fmt.Sprintf("%032b", long)
	octets := make([]string, 4)
	for i := 0; i < 4; i++ {
		val, _ := strconv.ParseInt(binary[i*8:(i+1)*8], 2, 64)
		octets[i] = strconv.FormatInt(val, 10)
	}
	return strings.Join(octets, ".")
}

// CidrToLong converts a CIDR prefix length (0-32) to its subnet mask as a 32-bit number.
func CidrToLong(cidr string) ([2]int64, error) {
	// This function matches the TS signature: cidrToLong(cidr: string) => [startLong, endLong]
	// Parse CIDR notation like "192.168.1.0/24"
	parts := strings.Split(cidr, "/")
	if len(parts) != 2 {
		return [2]int64{}, fmt.Errorf("Invalid CIDR notation")
	}

	ipStr := parts[0]
	prefixStr := parts[1]

	prefix, err := strconv.Atoi(prefixStr)
	if err != nil || prefix < 0 || prefix > 32 {
		return [2]int64{}, fmt.Errorf("CIDR must be an integer between 0 and 32")
	}

	ipLong, err := IpToLong(ipStr)
	if err != nil {
		return [2]int64{}, err
	}

	// Calculate mask
	var mask int64
	if prefix == 0 {
		mask = 0
	} else {
		mask = int64(^uint32(0) << (32 - prefix))
	}

	start := ipLong & mask
	// End = start | ~mask (all host bits set to 1)
	end := start | (int64(0xFFFFFFFF) &^ mask)

	return [2]int64{start, end}, nil
}

// cidrPrefixToLong converts a CIDR prefix length to a subnet mask long value.
// This is an internal helper matching the TS cidrToLong(cidr: number) function.
func cidrPrefixToLong(cidr int) (int64, error) {
	if cidr < 0 || cidr > 32 {
		return 0, fmt.Errorf("CIDR must be an integer between 0 and 32")
	}
	if cidr == 0 {
		return 0, nil
	}
	// Build binary string of cidr 1s followed by (32-cidr) 0s
	binStr := strings.Repeat("1", cidr) + strings.Repeat("0", 32-cidr)
	val, _ := strconv.ParseInt(binStr, 2, 64)
	return val, nil
}

// CidrToSubnetMask converts a CIDR prefix length (0-32) to a subnet mask string.
func CidrToSubnetMask(cidr int) string {
	maskLong, err := cidrPrefixToLong(cidr)
	if err != nil {
		panic(err.Error())
	}
	return LongToIp(maskLong)
}

// SubnetMaskToCidr converts a subnet mask string to its CIDR prefix length.
func SubnetMaskToCidr(mask string) (int, error) {
	if mask == "" {
		return 0, fmt.Errorf("Subnet mask is required")
	}

	octets := strings.Split(mask, ".")
	if len(octets) != 4 {
		return 0, fmt.Errorf("Invalid subnet mask format")
	}

	var sb strings.Builder
	for _, octet := range octets {
		num, err := strconv.Atoi(octet)
		if err != nil || num < 0 || num > 255 {
			return 0, fmt.Errorf("Invalid subnet mask format")
		}
		sb.WriteString(fmt.Sprintf("%08b", num))
	}

	binaryString := sb.String()

	// Check that binary string is consecutive 1s followed by 0s
	matched, _ := regexp.MatchString(`^1*0*$`, binaryString)
	if !matched {
		return 0, fmt.Errorf("Invalid subnet mask: must be consecutive 1s followed by 0s")
	}

	// Count 1s
	cidr := strings.Count(binaryString, "1")
	return cidr, nil
}

// GetNetworkAddress calculates the network address from an IP and subnet mask.
// Returns the network address as a 32-bit unsigned integer.
func GetNetworkAddress(ipStr string, mask string) (string, error) {
	if ipStr == "" {
		return "", fmt.Errorf("IP address is required")
	}
	if mask == "" {
		return "", fmt.Errorf("Subnet mask is required")
	}

	// Validate IP format
	ipParts := strings.Split(ipStr, ".")
	if len(ipParts) != 4 {
		return "", fmt.Errorf("Invalid IP address or subnet mask")
	}
	for _, part := range ipParts {
		num, err := strconv.Atoi(part)
		if err != nil || num < 0 || num > 255 {
			return "", fmt.Errorf("Invalid IP address or subnet mask")
		}
	}

	// Validate mask format
	maskParts := strings.Split(mask, ".")
	if len(maskParts) != 4 {
		return "", fmt.Errorf("Invalid IP address or subnet mask")
	}
	for _, part := range maskParts {
		num, err := strconv.Atoi(part)
		if err != nil || num < 0 || num > 255 {
			return "", fmt.Errorf("Invalid IP address or subnet mask")
		}
	}

	ipLong, err := IpToLong(ipStr)
	if err != nil {
		return "", fmt.Errorf("Invalid IP address or subnet mask")
	}

	cidr, err := SubnetMaskToCidr(mask)
	if err != nil {
		return "", fmt.Errorf("Invalid IP address or subnet mask")
	}

	maskLong, err := cidrPrefixToLong(cidr)
	if err != nil {
		return "", fmt.Errorf("Invalid IP address or subnet mask")
	}

	networkAddress := ipLong & maskLong
	return LongToIp(networkAddress), nil
}

// GetIpClass returns the IP class (A, B, C, D, E) for an IPv4 address.
// Returns empty string for invalid or reserved addresses.
func GetIpClass(ipStr string) (string, error) {
	if ipStr == "" {
		return "", nil
	}

	parts := strings.Split(ipStr, ".")
	if len(parts) != 4 {
		return "", nil
	}

	firstOctet, err := strconv.Atoi(parts[0])
	if err != nil || firstOctet < 0 || firstOctet > 255 {
		return "", nil
	}

	if firstOctet == 0 {
		return "", nil // Reserved
	}
	if firstOctet < 128 {
		return "A", nil // 1-127
	}
	if firstOctet < 192 {
		return "B", nil // 128-191
	}
	if firstOctet < 224 {
		return "C", nil // 192-223
	}
	if firstOctet < 240 {
		return "D", nil // 224-239
	}
	return "E", nil // 240-255
}

// IsInRange checks if an IP address is within a specified CIDR range.
// The cidr parameter is a string in "network/prefix" format, e.g. "192.168.1.0/24".
func IsInRange(ipStr string, cidr string) (bool, error) {
	if ipStr == "" {
		return false, fmt.Errorf("Remote IP address is required")
	}

	parts := strings.Split(cidr, "/")
	if len(parts) != 2 {
		return false, fmt.Errorf("Invalid CIDR notation")
	}

	networkIp := parts[0]
	if networkIp == "" {
		return false, fmt.Errorf("Network IP address is required")
	}

	prefix, err := strconv.Atoi(parts[1])
	if err != nil || prefix < 0 || prefix > 32 {
		return false, fmt.Errorf("CIDR must be an integer between 0 and 32")
	}

	remoteLong, err := IpToLong(ipStr)
	if err != nil {
		return false, fmt.Errorf("Invalid IP address format: %v", err)
	}

	networkLong, err := IpToLong(networkIp)
	if err != nil {
		return false, fmt.Errorf("Invalid IP address format: %v", err)
	}

	// Special cases
	if prefix == 0 {
		return true, nil // All IPs are in range
	}
	if prefix == 32 {
		return remoteLong == networkLong, nil // Exact match required
	}

	// Normal case
	shift := 32 - prefix
	mask := int64(0xFFFFFFFF) << shift & 0xFFFFFFFF
	return (remoteLong & mask) == (networkLong & mask), nil
}

// IsPrivateIp checks if an IP address is within private IP ranges
// (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16).
func IsPrivateIp(ipStr string) bool {
	if ipStr == "" {
		return false
	}

	privateRanges := []struct {
		network string
		cidr    int
	}{
		{"10.0.0.0", 8},
		{"172.16.0.0", 12},
		{"192.168.0.0", 16},
	}

	remoteLong, err := IpToLong(ipStr)
	if err != nil {
		return false
	}

	for _, r := range privateRanges {
		networkLong, _ := IpToLong(r.network)
		shift := 32 - r.cidr
		mask := int64(0xFFFFFFFF) << shift & 0xFFFFFFFF
		if (remoteLong & mask) == (networkLong & mask) {
			return true
		}
	}

	return false
}
