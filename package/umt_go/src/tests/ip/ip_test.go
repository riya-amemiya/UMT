package ip_test

import (
	"fmt"
	"strconv"
	"strings"
	"testing"

	"github.com/riya-amemiya/umt-go/src/ip"
)

// assertPanic checks that f panics with a message containing expectedMsg.
func assertPanic(t *testing.T, f func(), expectedMsg string) {
	t.Helper()
	defer func() {
		r := recover()
		if r == nil {
			t.Errorf("expected panic containing %q, but did not panic", expectedMsg)
			return
		}
		msg := fmt.Sprintf("%v", r)
		if !strings.Contains(msg, expectedMsg) {
			t.Errorf("expected panic message containing %q, got %q", expectedMsg, msg)
		}
	}()
	f()
}

// =============================================================================
// IpToBinaryString
// =============================================================================

func TestIpToBinaryString(t *testing.T) {
	tests := []struct {
		ipAddr   string
		expected string
	}{
		{"192.168.0.1", "11000000101010000000000000000001"},
		{"0.0.0.0", "00000000000000000000000000000000"},
		{"255.255.255.255", "11111111111111111111111111111111"},
		{"1.2.3.4", "00000001000000100000001100000100"},
		{"10.0.0.1", "00001010000000000000000000000001"},
		{"172.16.0.1", "10101100000100000000000000000001"},
		{"127.0.0.1", "01111111000000000000000000000001"},
		{"169.254.0.1", "10101001111111100000000000000001"},
	}
	for _, tt := range tests {
		t.Run(tt.ipAddr, func(t *testing.T) {
			result, err := ip.IpToBinaryString(tt.ipAddr)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if result != tt.expected {
				t.Errorf("IpToBinaryString(%q) = %q, want %q", tt.ipAddr, result, tt.expected)
			}
		})
	}
}

func TestIpToBinaryStringEdgeCases(t *testing.T) {
	tests := []struct {
		ipAddr   string
		expected string
	}{
		{"0.0.0.0", "00000000000000000000000000000000"},
		{"1.1.1.1", "00000001000000010000000100000001"},
		{"255.255.255.255", "11111111111111111111111111111111"},
		{"128.0.0.0", "10000000000000000000000000000000"},
		{"0.255.0.255", "00000000111111110000000011111111"},
	}
	for _, tt := range tests {
		t.Run(tt.ipAddr, func(t *testing.T) {
			result, err := ip.IpToBinaryString(tt.ipAddr)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if result != tt.expected {
				t.Errorf("IpToBinaryString(%q) = %q, want %q", tt.ipAddr, result, tt.expected)
			}
		})
	}
}

func TestIpToBinaryStringInvalid(t *testing.T) {
	tests := []struct {
		ipAddr      string
		errContains string
	}{
		{"", "IP address is required"},
		{"192.168", "Invalid IP address format"},
		{"a.b.c.d", "Invalid IP address format"},
		{"256.1.2.3", "Invalid IP address format"},
		{"-1.1.1.1", "Invalid IP address format"},
		{"1.2.3.4.5", "Invalid IP address format"},
		{"192.168.1", "Invalid IP address format"},
		{"192.168.1.1.1", "Invalid IP address format"},
		{"192.168.1.", "Invalid IP address format"},
		{"192.168..1", "Invalid IP address format"},
		{".192.168.1", "Invalid IP address format"},
		{"192,168,1,1", "Invalid IP address format"},
		{"192.168.1.1.", "Invalid IP address format"},
		{"192.168.1.+1", "Invalid IP address format"},
		{"256.256.256.256", "Invalid IP address format"},
		{"999.999.999.999", "Invalid IP address format"},
	}
	for _, tt := range tests {
		t.Run(tt.ipAddr, func(t *testing.T) {
			_, err := ip.IpToBinaryString(tt.ipAddr)
			if err == nil {
				t.Fatalf("expected error for %q, got nil", tt.ipAddr)
			}
			if !strings.Contains(err.Error(), tt.errContains) {
				t.Errorf("expected error containing %q, got %q", tt.errContains, err.Error())
			}
		})
	}
}

func TestIpToBinaryStringLeadingZeros(t *testing.T) {
	invalids := []string{
		"0000", "00.00", "0.0.0",
		"192.168.01.1", "192.168.1.01", "010.020.030.040",
	}
	for _, ipAddr := range invalids {
		t.Run(ipAddr, func(t *testing.T) {
			_, err := ip.IpToBinaryString(ipAddr)
			if err == nil {
				t.Fatalf("expected error for %q, got nil", ipAddr)
			}
			if !strings.Contains(err.Error(), "Invalid IP address format") {
				t.Errorf("expected 'Invalid IP address format', got %q", err.Error())
			}
		})
	}
}

// =============================================================================
// IpToLong
// =============================================================================

func TestIpToLong(t *testing.T) {
	tests := []struct {
		ipAddr   string
		expected int64
	}{
		{"192.168.0.1", 0xC0A80001},
		{"128.0.0.1", 0x80000001},
		{"10.0.0.1", 0x0A000001},
		{"172.16.0.1", 0xAC100001},
		{"255.255.255.255", 0xFFFFFFFF},
		{"0.0.0.0", 0x00000000},
		{"127.0.0.1", 0x7F000001},
		{"1.2.3.4", 0x01020304},
	}
	for _, tt := range tests {
		t.Run(tt.ipAddr, func(t *testing.T) {
			result, err := ip.IpToLong(tt.ipAddr)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if result != tt.expected {
				t.Errorf("IpToLong(%q) = %d, want %d", tt.ipAddr, result, tt.expected)
			}
		})
	}
}

func TestIpToLongInvalid(t *testing.T) {
	tests := []struct {
		ipAddr      string
		errContains string
	}{
		{"", "IP address is required"},
		{"192.168", "Invalid IP address format"},
		{"256.1.2.3", "Invalid IP address format"},
		{"a.b.c.d", "Invalid IP address format"},
		{"-1.0.0.0", "Invalid IP address format"},
		{"192.168.1.1.1", "Invalid IP address format"},
		{"192.168..1", "Invalid IP address format"},
	}
	for _, tt := range tests {
		t.Run(tt.ipAddr, func(t *testing.T) {
			_, err := ip.IpToLong(tt.ipAddr)
			if err == nil {
				t.Fatalf("expected error for %q, got nil", tt.ipAddr)
			}
			if !strings.Contains(err.Error(), tt.errContains) {
				t.Errorf("expected error containing %q, got %q", tt.errContains, err.Error())
			}
		})
	}
}

// =============================================================================
// LongToIp
// =============================================================================

func TestLongToIp(t *testing.T) {
	tests := []struct {
		long     int64
		expected string
	}{
		{0xC0A80001, "192.168.0.1"},
		{0x80000001, "128.0.0.1"},
		{0x0A000001, "10.0.0.1"},
		{0xAC100001, "172.16.0.1"},
		{0xFFFFFFFF, "255.255.255.255"},
		{0x00000000, "0.0.0.0"},
		{0x7F000001, "127.0.0.1"},
		{0x01020304, "1.2.3.4"},
	}
	for _, tt := range tests {
		t.Run(tt.expected, func(t *testing.T) {
			result := ip.LongToIp(tt.long)
			if result != tt.expected {
				t.Errorf("LongToIp(%d) = %q, want %q", tt.long, result, tt.expected)
			}
		})
	}
}

func TestLongToIpInvalid(t *testing.T) {
	tests := []int64{-1, 0x100000000}
	for _, v := range tests {
		t.Run(fmt.Sprintf("%d", v), func(t *testing.T) {
			assertPanic(t, func() { ip.LongToIp(v) }, "Input must be a valid 32-bit unsigned integer")
		})
	}
}

// =============================================================================
// CidrToLong (string CIDR notation -> [start, end])
// =============================================================================

func TestCidrToLong(t *testing.T) {
	tests := []struct {
		cidr  string
		start int64
		end   int64
	}{
		{"192.168.1.0/24", 0xC0A80100, 0xC0A801FF},
		{"10.0.0.0/8", 0x0A000000, 0x0AFFFFFF},
		{"172.16.0.0/16", 0xAC100000, 0xAC10FFFF},
		{"192.168.1.0/32", 0xC0A80100, 0xC0A80100},
		{"0.0.0.0/0", 0x00000000, 0xFFFFFFFF},
	}
	for _, tt := range tests {
		t.Run(tt.cidr, func(t *testing.T) {
			result, err := ip.CidrToLong(tt.cidr)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if result[0] != tt.start || result[1] != tt.end {
				t.Errorf("CidrToLong(%q) = [%d, %d], want [%d, %d]",
					tt.cidr, result[0], result[1], tt.start, tt.end)
			}
		})
	}
}

func TestCidrToLongInvalid(t *testing.T) {
	tests := []struct {
		cidr        string
		errContains string
	}{
		{"192.168.1.0", "Invalid CIDR notation"},
		{"192.168.1.0/-1", "CIDR must be an integer between 0 and 32"},
		{"192.168.1.0/33", "CIDR must be an integer between 0 and 32"},
		{"invalid/24", "Invalid IP address format"},
	}
	for _, tt := range tests {
		t.Run(tt.cidr, func(t *testing.T) {
			_, err := ip.CidrToLong(tt.cidr)
			if err == nil {
				t.Fatalf("expected error for %q, got nil", tt.cidr)
			}
			if !strings.Contains(err.Error(), tt.errContains) {
				t.Errorf("expected error containing %q, got %q", tt.errContains, err.Error())
			}
		})
	}
}

// =============================================================================
// CidrToSubnetMask
// =============================================================================

func TestCidrToSubnetMask(t *testing.T) {
	tests := []struct {
		cidr     int
		expected string
	}{
		{32, "255.255.255.255"},
		{24, "255.255.255.0"},
		{16, "255.255.0.0"},
		{8, "255.0.0.0"},
		{0, "0.0.0.0"},
		{1, "128.0.0.0"},
		{31, "255.255.255.254"},
		{28, "255.255.255.240"},
		{20, "255.255.240.0"},
	}
	for _, tt := range tests {
		t.Run(fmt.Sprintf("/%d", tt.cidr), func(t *testing.T) {
			result := ip.CidrToSubnetMask(tt.cidr)
			if result != tt.expected {
				t.Errorf("CidrToSubnetMask(%d) = %q, want %q", tt.cidr, result, tt.expected)
			}
		})
	}
}

func TestCidrToSubnetMaskInvalid(t *testing.T) {
	tests := []int{-1, 33}
	for _, cidr := range tests {
		t.Run(fmt.Sprintf("%d", cidr), func(t *testing.T) {
			assertPanic(t, func() { ip.CidrToSubnetMask(cidr) }, "CIDR must be an integer between 0 and 32")
		})
	}
}

// =============================================================================
// SubnetMaskToCidr
// =============================================================================

func TestSubnetMaskToCidr(t *testing.T) {
	tests := []struct {
		mask     string
		expected int
	}{
		{"255.255.255.255", 32},
		{"255.255.255.0", 24},
		{"255.255.0.0", 16},
		{"255.0.0.0", 8},
		{"255.255.254.0", 23},
		{"255.255.255.252", 30},
		{"255.255.255.248", 29},
		{"255.255.240.0", 20},
		{"0.0.0.0", 0},
	}
	for _, tt := range tests {
		t.Run(tt.mask, func(t *testing.T) {
			result, err := ip.SubnetMaskToCidr(tt.mask)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if result != tt.expected {
				t.Errorf("SubnetMaskToCidr(%q) = %d, want %d", tt.mask, result, tt.expected)
			}
		})
	}
}

func TestSubnetMaskToCidrInvalid(t *testing.T) {
	tests := []struct {
		mask        string
		errContains string
	}{
		{"", "Subnet mask is required"},
		{"192.168", "Invalid subnet mask format"},
		{"256.255.255.0", "Invalid subnet mask format"},
		{"255.255.255.256", "Invalid subnet mask format"},
		{"255.-1.255.0", "Invalid subnet mask format"},
		{"255.255.255.abc", "Invalid subnet mask format"},
		{"a.b.c.d", "Invalid subnet mask format"},
		{"255.255.255.1", "Invalid subnet mask: must be consecutive 1s followed by 0s"},
		{"255.0.255.0", "Invalid subnet mask: must be consecutive 1s followed by 0s"},
		{"254.255.255.0", "Invalid subnet mask: must be consecutive 1s followed by 0s"},
	}
	for _, tt := range tests {
		t.Run(tt.mask, func(t *testing.T) {
			_, err := ip.SubnetMaskToCidr(tt.mask)
			if err == nil {
				t.Fatalf("expected error for %q, got nil", tt.mask)
			}
			if !strings.Contains(err.Error(), tt.errContains) {
				t.Errorf("expected error containing %q, got %q", tt.errContains, err.Error())
			}
		})
	}
}

// =============================================================================
// GetNetworkAddress
// =============================================================================

func TestGetNetworkAddress(t *testing.T) {
	tests := []struct {
		ipAddr   string
		mask     string
		expected string
	}{
		{"192.168.1.1", "255.255.255.0", "192.168.1.0"},
		{"172.16.5.1", "255.255.0.0", "172.16.0.0"},
		{"10.0.0.15", "255.0.0.0", "10.0.0.0"},
		{"192.168.1.1", "255.255.254.0", "192.168.0.0"},
		{"255.255.255.255", "255.255.255.0", "255.255.255.0"},
		{"0.0.0.0", "255.255.255.0", "0.0.0.0"},
		{"192.168.1.1", "255.255.255.252", "192.168.1.0"},
		{"10.10.10.10", "255.255.255.240", "10.10.10.0"},
	}
	for _, tt := range tests {
		t.Run(tt.ipAddr+"_"+tt.mask, func(t *testing.T) {
			result, err := ip.GetNetworkAddress(tt.ipAddr, tt.mask)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if result != tt.expected {
				t.Errorf("GetNetworkAddress(%q, %q) = %q, want %q", tt.ipAddr, tt.mask, result, tt.expected)
			}
		})
	}
}

func TestGetNetworkAddressInvalid(t *testing.T) {
	tests := []struct {
		ipAddr      string
		mask        string
		errContains string
	}{
		{"", "255.255.255.0", "IP address is required"},
		{"192.168.1.1", "", "Subnet mask is required"},
		{"invalid", "255.255.255.0", "Invalid IP address or subnet mask"},
		{"192.168.1.1", "invalid", "Invalid IP address or subnet mask"},
		{"256.256.256.256", "255.255.255.0", "Invalid IP address or subnet mask"},
		{"192.168.1.1", "256.256.256.256", "Invalid IP address or subnet mask"},
		{"192.168.1.1", "255.255.128.3", "Invalid IP address or subnet mask"},
	}
	for _, tt := range tests {
		t.Run(tt.ipAddr+"_"+tt.mask, func(t *testing.T) {
			_, err := ip.GetNetworkAddress(tt.ipAddr, tt.mask)
			if err == nil {
				t.Fatalf("expected error for ip=%q mask=%q, got nil", tt.ipAddr, tt.mask)
			}
			if !strings.Contains(err.Error(), tt.errContains) {
				t.Errorf("expected error containing %q, got %q", tt.errContains, err.Error())
			}
		})
	}
}

// =============================================================================
// GetIpClass
// =============================================================================

func TestGetIpClass(t *testing.T) {
	tests := []struct {
		ipAddr   string
		expected string
	}{
		{"1.0.0.0", "A"},
		{"126.0.0.1", "A"},
		{"128.0.0.1", "B"},
		{"191.255.0.1", "B"},
		{"192.0.0.1", "C"},
		{"223.255.0.1", "C"},
		{"224.0.0.1", "D"},
		{"239.0.0.1", "D"},
		{"240.0.0.1", "E"},
		{"255.255.255.255", "E"},
	}
	for _, tt := range tests {
		t.Run(tt.ipAddr, func(t *testing.T) {
			result, err := ip.GetIpClass(tt.ipAddr)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if result != tt.expected {
				t.Errorf("GetIpClass(%q) = %q, want %q", tt.ipAddr, result, tt.expected)
			}
		})
	}
}

func TestGetIpClassInvalid(t *testing.T) {
	invalids := []string{
		"",
		"0.0.0.0",
		"256.0.0.1",
		"192.168",
		"192.168.1.1.1",
		"a.b.c.d",
		"192.168.1",
		"-1.0.0.0",
		"1.2.3.4.5",
	}
	for _, ipAddr := range invalids {
		t.Run(ipAddr, func(t *testing.T) {
			result, _ := ip.GetIpClass(ipAddr)
			if result != "" {
				t.Errorf("GetIpClass(%q) = %q, want empty string", ipAddr, result)
			}
		})
	}
}

// =============================================================================
// IsInRange
// =============================================================================

func TestIsInRange(t *testing.T) {
	tests := []struct {
		remoteIp  string
		networkIp string
		cidr      int
		expected  bool
	}{
		{"192.168.1.2", "192.168.1.0", 24, true},
		{"192.168.2.2", "192.168.1.0", 24, false},
		{"10.0.0.5", "10.0.0.0", 8, true},
		{"11.0.0.5", "10.0.0.0", 8, false},
		{"172.16.1.1", "172.16.0.0", 16, true},
		{"172.17.1.1", "172.16.0.0", 16, false},
		{"192.168.1.0", "192.168.1.0", 32, true},
		{"192.168.1.1", "192.168.1.1", 32, true},
		{"192.168.1.1", "192.168.1.0", 32, false},
		{"192.168.1.1", "192.168.1.0", 30, true},
		{"192.168.1.4", "192.168.1.0", 30, false},
		{"0.0.0.1", "0.0.0.0", 0, true},
		{"255.255.255.255", "0.0.0.0", 0, true},
	}
	for _, tt := range tests {
		cidrStr := fmt.Sprintf("%s/%d", tt.networkIp, tt.cidr)
		t.Run(fmt.Sprintf("%s_in_%s", tt.remoteIp, cidrStr), func(t *testing.T) {
			result, err := ip.IsInRange(tt.remoteIp, cidrStr)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if result != tt.expected {
				t.Errorf("IsInRange(%q, %q) = %v, want %v", tt.remoteIp, cidrStr, result, tt.expected)
			}
		})
	}
}

func TestIsInRangeBoundary(t *testing.T) {
	result, err := ip.IsInRange("192.168.1.1", "0.0.0.0/0")
	if err != nil || !result {
		t.Errorf("CIDR 0: expected true, got %v err=%v", result, err)
	}
	result, err = ip.IsInRange("192.168.1.0", "192.168.1.0/32")
	if err != nil || !result {
		t.Errorf("CIDR 32 exact: expected true, got %v err=%v", result, err)
	}
	result, err = ip.IsInRange("192.168.1.1", "192.168.1.0/31")
	if err != nil || !result {
		t.Errorf("CIDR 31: expected true, got %v err=%v", result, err)
	}
	result, err = ip.IsInRange("192.168.1.1", "192.168.1.0/1")
	if err != nil || !result {
		t.Errorf("CIDR 1: expected true, got %v err=%v", result, err)
	}
}

func TestIsInRangeEdgeCases(t *testing.T) {
	r, _ := ip.IsInRange("192.168.1.5", "192.168.1.0/24")
	if !r {
		t.Error("expected true for 192.168.1.5 in /24")
	}
	r, _ = ip.IsInRange("192.168.2.0", "192.168.1.0/24")
	if r {
		t.Error("expected false for 192.168.2.0 in /24")
	}
	r, _ = ip.IsInRange("192.168.1.0", "192.168.1.0/31")
	if !r {
		t.Error("expected true for .0 in /31")
	}
	r, _ = ip.IsInRange("192.168.1.1", "192.168.1.0/31")
	if !r {
		t.Error("expected true for .1 in /31")
	}
	r, _ = ip.IsInRange("192.168.1.2", "192.168.1.0/31")
	if r {
		t.Error("expected false for .2 in /31")
	}
	r, _ = ip.IsInRange("192.168.1.3", "192.168.1.0/30")
	if !r {
		t.Error("expected true for .3 in /30")
	}
	r, _ = ip.IsInRange("192.168.1.4", "192.168.1.4/30")
	if !r {
		t.Error("expected true for .4 in .4/30")
	}
}

func TestIsInRangeInvalidInputs(t *testing.T) {
	tests := []struct {
		remoteIp    string
		cidr        string
		errContains string
	}{
		{"", "192.168.1.0/24", "Remote IP address is required"},
		{"192.168.1.1", "/24", "Network IP address is required"},
		{"192.168.1.1", "192.168.1.0/-1", "CIDR must be an integer between 0 and 32"},
		{"192.168.1.1", "192.168.1.0/33", "CIDR must be an integer between 0 and 32"},
	}
	for _, tt := range tests {
		t.Run(tt.remoteIp+"_"+tt.cidr, func(t *testing.T) {
			_, err := ip.IsInRange(tt.remoteIp, tt.cidr)
			if err == nil {
				t.Fatalf("expected error, got nil")
			}
			if !strings.Contains(err.Error(), tt.errContains) {
				t.Errorf("expected error containing %q, got %q", tt.errContains, err.Error())
			}
		})
	}
}

func TestIsInRangeInvalidIPs(t *testing.T) {
	tests := []struct {
		remoteIp string
		cidr     string
	}{
		{"invalid", "192.168.1.0/24"},
		{"192.168.1.1", "invalid/24"},
		{"256.256.256.256", "192.168.1.0/24"},
		{"192.168.1.1", "256.256.256.256/24"},
		{"192.168.1.1.1", "192.168.1.0/24"},
		{"192.168.-1.1", "192.168.1.0/24"},
		{"192.168.1", "192.168.1.0/24"},
	}
	for _, tt := range tests {
		t.Run(tt.remoteIp+"_"+tt.cidr, func(t *testing.T) {
			_, err := ip.IsInRange(tt.remoteIp, tt.cidr)
			if err == nil {
				t.Fatalf("expected error for invalid IP, got nil")
			}
			if !strings.Contains(err.Error(), "Invalid IP address format") {
				t.Errorf("expected 'Invalid IP address format', got %q", err.Error())
			}
		})
	}
}

func TestIsInRangeErrorMessageFormatting(t *testing.T) {
	_, err := ip.IsInRange("invalid-ip", "192.168.1.0/24")
	if err == nil {
		t.Fatal("expected error")
	}
	if !strings.Contains(err.Error(), "Invalid IP address format") {
		t.Errorf("expected 'Invalid IP address format' in error, got %q", err.Error())
	}

	_, err = ip.IsInRange("192.168.1.1", "invalid-network/24")
	if err == nil {
		t.Fatal("expected error")
	}
	if !strings.Contains(err.Error(), "Invalid IP address format") {
		t.Errorf("expected 'Invalid IP address format' in error, got %q", err.Error())
	}

	_, err = ip.IsInRange("999.999.999.999", "192.168.1.0/24")
	if err == nil {
		t.Fatal("expected error")
	}
	if !strings.Contains(err.Error(), "Invalid IP address format") {
		t.Errorf("expected 'Invalid IP address format' in error, got %q", err.Error())
	}
}

// =============================================================================
// IsPrivateIp
// =============================================================================

func TestIsPrivateIpTrue(t *testing.T) {
	privateIps := []string{
		"10.0.0.1",
		"10.255.255.255",
		"172.16.0.1",
		"172.31.255.255",
		"192.168.0.1",
		"192.168.255.255",
	}
	for _, ipAddr := range privateIps {
		t.Run(ipAddr, func(t *testing.T) {
			if !ip.IsPrivateIp(ipAddr) {
				t.Errorf("IsPrivateIp(%q) = false, want true", ipAddr)
			}
		})
	}
}

func TestIsPrivateIpFalse(t *testing.T) {
	nonPrivateIps := []string{
		"9.255.255.255",
		"11.0.0.0",
		"172.15.255.255",
		"172.32.0.0",
		"192.167.255.255",
		"192.169.0.0",
		"8.8.8.8",
		"1.1.1.1",
		"169.254.0.1",
		"127.0.0.1",
	}
	for _, ipAddr := range nonPrivateIps {
		t.Run(ipAddr, func(t *testing.T) {
			if ip.IsPrivateIp(ipAddr) {
				t.Errorf("IsPrivateIp(%q) = true, want false", ipAddr)
			}
		})
	}
}

func TestIsPrivateIpInvalid(t *testing.T) {
	invalids := []string{
		"",
		"256.256.256.256",
		"192.168",
		"a.b.c.d",
		"192.168.1.1.1",
		"-1.0.0.0",
	}
	for _, ipAddr := range invalids {
		t.Run(ipAddr, func(t *testing.T) {
			if ip.IsPrivateIp(ipAddr) {
				t.Errorf("IsPrivateIp(%q) = true, want false for invalid input", ipAddr)
			}
		})
	}
}

// =============================================================================
// Integration: Round-trip conversions
// =============================================================================

func TestRoundTripIpLong(t *testing.T) {
	ips := []string{
		"192.168.1.1", "10.0.0.1", "172.16.0.1",
		"8.8.8.8", "255.255.255.255", "0.0.0.0",
	}
	for _, ipAddr := range ips {
		longVal, err := ip.IpToLong(ipAddr)
		if err != nil {
			t.Fatalf("IpToLong(%q) error: %v", ipAddr, err)
		}
		converted := ip.LongToIp(longVal)
		if converted != ipAddr {
			t.Errorf("round-trip failed: %q -> %d -> %q", ipAddr, longVal, converted)
		}
	}
}

func TestRoundTripCidrSubnetMask(t *testing.T) {
	cidrs := []int{8, 16, 24, 30, 32}
	for _, cidr := range cidrs {
		mask := ip.CidrToSubnetMask(cidr)
		result, err := ip.SubnetMaskToCidr(mask)
		if err != nil {
			t.Fatalf("SubnetMaskToCidr(%q) error: %v", mask, err)
		}
		if result != cidr {
			t.Errorf("round-trip failed: /%d -> %q -> /%d", cidr, mask, result)
		}
	}
}

func TestIntegrationNetworkCalculations(t *testing.T) {
	testCases := []struct {
		ipAddr          string
		subnet          string
		cidr            int
		expectedNetwork string
	}{
		{"192.168.1.100", "255.255.255.0", 24, "192.168.1.0"},
		{"10.0.15.200", "255.255.0.0", 16, "10.0.0.0"},
		{"172.16.5.10", "255.255.255.240", 28, "172.16.5.0"},
	}

	for _, tc := range testCases {
		binaryStr, err := ip.IpToBinaryString(tc.ipAddr)
		if err != nil {
			t.Fatalf("IpToBinaryString error: %v", err)
		}
		if len(binaryStr) != 32 {
			t.Errorf("binary string length = %d, want 32", len(binaryStr))
		}

		_, err = ip.GetNetworkAddress(tc.ipAddr, tc.subnet)
		if err != nil {
			t.Fatalf("GetNetworkAddress error: %v", err)
		}

		convertedSubnet := ip.CidrToSubnetMask(tc.cidr)
		if convertedSubnet != tc.subnet {
			t.Errorf("CidrToSubnetMask(%d) = %q, want %q", tc.cidr, convertedSubnet, tc.subnet)
		}

		cidrStr := fmt.Sprintf("%s/%d", tc.expectedNetwork, tc.cidr)
		inRange, err := ip.IsInRange(tc.ipAddr, cidrStr)
		if err != nil {
			t.Fatalf("IsInRange error: %v", err)
		}
		if !inRange {
			t.Errorf("%s should be in range %s", tc.ipAddr, cidrStr)
		}
	}
}

func TestIntegrationPrivateIpClasses(t *testing.T) {
	tests := []struct {
		ipAddr    string
		isPrivate bool
		class     string
	}{
		{"192.168.1.1", true, "C"},
		{"10.0.0.1", true, "A"},
		{"172.16.1.1", true, "B"},
		{"8.8.8.8", false, "A"},
		{"1.1.1.1", false, "A"},
	}
	for _, tt := range tests {
		if ip.IsPrivateIp(tt.ipAddr) != tt.isPrivate {
			t.Errorf("IsPrivateIp(%q) = %v, want %v", tt.ipAddr, !tt.isPrivate, tt.isPrivate)
		}
		class, _ := ip.GetIpClass(tt.ipAddr)
		if class != tt.class {
			t.Errorf("GetIpClass(%q) = %q, want %q", tt.ipAddr, class, tt.class)
		}
		longVal, _ := ip.IpToLong(tt.ipAddr)
		convertedIp := ip.LongToIp(longVal)
		if ip.IsPrivateIp(convertedIp) != tt.isPrivate {
			t.Errorf("round-trip IsPrivateIp(%q) mismatch", convertedIp)
		}
		convertedClass, _ := ip.GetIpClass(convertedIp)
		if convertedClass != tt.class {
			t.Errorf("round-trip GetIpClass(%q) = %q, want %q", convertedIp, convertedClass, tt.class)
		}
	}
}

func TestIntegrationComplexNetworkRanges(t *testing.T) {
	type ipTest struct {
		ipAddr  string
		inRange bool
	}
	tests := []struct {
		network string
		cidr    int
		testIps []ipTest
	}{
		{
			"192.168.1.0", 24,
			[]ipTest{
				{"192.168.1.1", true},
				{"192.168.1.255", true},
				{"192.168.2.1", false},
				{"192.168.0.255", false},
			},
		},
		{
			"10.0.0.0", 8,
			[]ipTest{
				{"10.255.255.255", true},
				{"11.0.0.1", false},
				{"10.192.168.1", true},
			},
		},
	}
	for _, tt := range tests {
		subnetMask := ip.CidrToSubnetMask(tt.cidr)
		convertedCidr, err := ip.SubnetMaskToCidr(subnetMask)
		if err != nil {
			t.Fatal(err)
		}
		if convertedCidr != tt.cidr {
			t.Errorf("SubnetMaskToCidr round-trip: got %d, want %d", convertedCidr, tt.cidr)
		}

		for _, ipTest := range tt.testIps {
			cidrStr := fmt.Sprintf("%s/%d", tt.network, tt.cidr)
			result, err := ip.IsInRange(ipTest.ipAddr, cidrStr)
			if err != nil {
				t.Fatal(err)
			}
			if result != ipTest.inRange {
				t.Errorf("IsInRange(%q, %q) = %v, want %v", ipTest.ipAddr, cidrStr, result, ipTest.inRange)
			}

			networkIp, err := ip.GetNetworkAddress(tt.network, subnetMask)
			if err != nil {
				t.Fatal(err)
			}
			cidrStr2 := fmt.Sprintf("%s/%d", networkIp, tt.cidr)
			result2, err := ip.IsInRange(ipTest.ipAddr, cidrStr2)
			if err != nil {
				t.Fatal(err)
			}
			if result2 != ipTest.inRange {
				t.Errorf("IsInRange(%q, %q) = %v, want %v", ipTest.ipAddr, cidrStr2, result2, ipTest.inRange)
			}
		}
	}
}

func TestIntegrationBinaryConversions(t *testing.T) {
	tests := []struct {
		ipAddr         string
		expectedBinary string
	}{
		{"192.168.1.1", "11000000101010000000000100000001"},
		{"255.255.255.255", "11111111111111111111111111111111"},
		{"0.0.0.0", "00000000000000000000000000000000"},
	}
	for _, tt := range tests {
		binaryStr, err := ip.IpToBinaryString(tt.ipAddr)
		if err != nil {
			t.Fatal(err)
		}
		if binaryStr != tt.expectedBinary {
			t.Errorf("IpToBinaryString(%q) = %q, want %q", tt.ipAddr, binaryStr, tt.expectedBinary)
		}

		longVal, err := ip.IpToLong(tt.ipAddr)
		if err != nil {
			t.Fatal(err)
		}
		binaryFromLong := fmt.Sprintf("%032b", longVal)
		if binaryFromLong != tt.expectedBinary {
			t.Errorf("binary from long = %q, want %q", binaryFromLong, tt.expectedBinary)
		}

		ipFromLong := ip.LongToIp(longVal)
		if ipFromLong != tt.ipAddr {
			t.Errorf("LongToIp(%d) = %q, want %q", longVal, ipFromLong, tt.ipAddr)
		}
	}
}

func TestIntegrationSubnetCalculations(t *testing.T) {
	subnets := []struct {
		network      string
		cidr         int
		expectedMask string
	}{
		{"192.168.1.0", 26, "255.255.255.192"},
		{"10.0.0.0", 12, "255.240.0.0"},
	}
	for _, s := range subnets {
		mask := ip.CidrToSubnetMask(s.cidr)
		if mask != s.expectedMask {
			t.Errorf("CidrToSubnetMask(%d) = %q, want %q", s.cidr, mask, s.expectedMask)
		}

		convertedCidr, err := ip.SubnetMaskToCidr(mask)
		if err != nil {
			t.Fatal(err)
		}
		if convertedCidr != s.cidr {
			t.Errorf("SubnetMaskToCidr(%q) = %d, want %d", mask, convertedCidr, s.cidr)
		}

		networkIp, err := ip.GetNetworkAddress(s.network, mask)
		if err != nil {
			t.Fatal(err)
		}

		cidrStr := fmt.Sprintf("%s/%d", networkIp, s.cidr)
		inRange, err := ip.IsInRange(s.network, cidrStr)
		if err != nil {
			t.Fatal(err)
		}
		if !inRange {
			t.Errorf("%s should be in range %s", s.network, cidrStr)
		}

		binaryNetwork, err := ip.IpToBinaryString(networkIp)
		if err != nil {
			t.Fatal(err)
		}
		binaryMask, err := ip.IpToBinaryString(mask)
		if err != nil {
			t.Fatal(err)
		}
		if len(binaryNetwork) != 32 || len(binaryMask) != 32 {
			t.Error("expected 32-bit binary strings")
		}

		maskOnes := strings.Count(binaryMask, "1")
		if maskOnes != s.cidr {
			t.Errorf("mask ones count = %d, want %d", maskOnes, s.cidr)
		}
	}
}

func TestIntegrationEdgeCases(t *testing.T) {
	tests := []struct {
		ipAddr    string
		class     string
		isPrivate bool
	}{
		{"127.0.0.1", "A", false},
		{"169.254.1.1", "B", false},
		{"192.168.0.1", "C", true},
	}
	for _, tt := range tests {
		longVal, err := ip.IpToLong(tt.ipAddr)
		if err != nil {
			t.Fatal(err)
		}
		binary, err := ip.IpToBinaryString(tt.ipAddr)
		if err != nil {
			t.Fatal(err)
		}
		reconstructed := ip.LongToIp(longVal)
		if reconstructed != tt.ipAddr {
			t.Errorf("reconstructed %q, want %q", reconstructed, tt.ipAddr)
		}

		class, _ := ip.GetIpClass(reconstructed)
		if class != tt.class {
			t.Errorf("GetIpClass(%q) = %q, want %q", reconstructed, class, tt.class)
		}
		if ip.IsPrivateIp(reconstructed) != tt.isPrivate {
			t.Errorf("IsPrivateIp(%q) = %v, want %v", reconstructed, !tt.isPrivate, tt.isPrivate)
		}

		if len(binary) != 32 {
			t.Errorf("binary string length = %d, want 32", len(binary))
		}
		parsedLong, err := strconv.ParseInt(binary, 2, 64)
		if err != nil {
			t.Fatal(err)
		}
		if parsedLong != longVal {
			t.Errorf("parsed binary = %d, want %d", parsedLong, longVal)
		}
	}
}
