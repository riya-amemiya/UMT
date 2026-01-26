package ip

import (
	"testing"
)

func TestIpToLong(t *testing.T) {
	tests := []struct {
		ip   string
		want int64
	}{
		{"0.0.0.0", 0},
		{"255.255.255.255", 4294967295},
		{"192.168.1.1", 3232235777},
		{"10.0.0.1", 167772161},
		{"172.16.0.1", 2886729729},
	}

	for _, tt := range tests {
		t.Run(tt.ip, func(t *testing.T) {
			got, err := IpToLong(tt.ip)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if got != tt.want {
				t.Errorf("IpToLong(%q) = %d, want %d", tt.ip, got, tt.want)
			}
		})
	}
}

func TestLongToIp(t *testing.T) {
	tests := []struct {
		long int64
		want string
	}{
		{0, "0.0.0.0"},
		{4294967295, "255.255.255.255"},
		{3232235777, "192.168.1.1"},
	}

	for _, tt := range tests {
		t.Run(tt.want, func(t *testing.T) {
			got := LongToIp(tt.long)
			if got != tt.want {
				t.Errorf("LongToIp(%d) = %q, want %q", tt.long, got, tt.want)
			}
		})
	}
}

func TestIpToBinaryString(t *testing.T) {
	got, err := IpToBinaryString("192.168.1.1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	want := "11000000101010000000000100000001"
	if got != want {
		t.Errorf("IpToBinaryString(\"192.168.1.1\") = %q, want %q", got, want)
	}
}

func TestCidrToSubnetMask(t *testing.T) {
	tests := []struct {
		cidr int
		want string
	}{
		{0, "0.0.0.0"},
		{8, "255.0.0.0"},
		{16, "255.255.0.0"},
		{24, "255.255.255.0"},
		{32, "255.255.255.255"},
	}

	for _, tt := range tests {
		t.Run(tt.want, func(t *testing.T) {
			got := CidrToSubnetMask(tt.cidr)
			if got != tt.want {
				t.Errorf("CidrToSubnetMask(%d) = %q, want %q", tt.cidr, got, tt.want)
			}
		})
	}
}

func TestSubnetMaskToCidr(t *testing.T) {
	tests := []struct {
		mask string
		want int
	}{
		{"0.0.0.0", 0},
		{"255.0.0.0", 8},
		{"255.255.0.0", 16},
		{"255.255.255.0", 24},
		{"255.255.255.255", 32},
	}

	for _, tt := range tests {
		t.Run(tt.mask, func(t *testing.T) {
			got, err := SubnetMaskToCidr(tt.mask)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if got != tt.want {
				t.Errorf("SubnetMaskToCidr(%q) = %d, want %d", tt.mask, got, tt.want)
			}
		})
	}
}

func TestGetIpClass(t *testing.T) {
	tests := []struct {
		ip   string
		want string
	}{
		{"1.0.0.0", "A"},
		{"127.0.0.0", "A"},
		{"128.0.0.0", "B"},
		{"191.255.255.255", "B"},
		{"192.0.0.0", "C"},
		{"223.255.255.255", "C"},
		{"224.0.0.0", "D"},
		{"239.255.255.255", "D"},
		{"240.0.0.0", "E"},
		{"255.255.255.255", "E"},
	}

	for _, tt := range tests {
		t.Run(tt.ip, func(t *testing.T) {
			got, _ := GetIpClass(tt.ip)
			if got != tt.want {
				t.Errorf("GetIpClass(%q) = %q, want %q", tt.ip, got, tt.want)
			}
		})
	}
}

func TestIsInRange(t *testing.T) {
	tests := []struct {
		ip   string
		cidr string
		want bool
	}{
		{"192.168.1.100", "192.168.1.0/24", true},
		{"192.168.2.1", "192.168.1.0/24", false},
		{"10.0.0.5", "10.0.0.0/8", true},
		{"172.16.5.5", "172.16.0.0/12", true},
	}

	for _, tt := range tests {
		t.Run(tt.ip+"_"+tt.cidr, func(t *testing.T) {
			got, err := IsInRange(tt.ip, tt.cidr)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if got != tt.want {
				t.Errorf("IsInRange(%q, %q) = %v, want %v", tt.ip, tt.cidr, got, tt.want)
			}
		})
	}
}

func TestIsPrivateIp(t *testing.T) {
	tests := []struct {
		ip   string
		want bool
	}{
		{"10.0.0.1", true},
		{"10.255.255.255", true},
		{"172.16.0.1", true},
		{"172.31.255.255", true},
		{"192.168.0.1", true},
		{"192.168.255.255", true},
		{"8.8.8.8", false},
		{"1.1.1.1", false},
	}

	for _, tt := range tests {
		t.Run(tt.ip, func(t *testing.T) {
			got := IsPrivateIp(tt.ip)
			if got != tt.want {
				t.Errorf("IsPrivateIp(%q) = %v, want %v", tt.ip, got, tt.want)
			}
		})
	}
}

func TestGetNetworkAddress(t *testing.T) {
	got, err := GetNetworkAddress("192.168.1.100", "255.255.255.0")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if got != "192.168.1.0" {
		t.Errorf("GetNetworkAddress(\"192.168.1.100\", \"255.255.255.0\") = %q, want \"192.168.1.0\"", got)
	}
}

func TestCidrToLong(t *testing.T) {
	result, err := CidrToLong("192.168.1.0/24")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	// start should be 192.168.1.0 = 3232235776
	// end should be 192.168.1.255 = 3232236031
	if result[0] != 3232235776 {
		t.Errorf("CidrToLong start = %d, want 3232235776", result[0])
	}
	if result[1] != 3232236031 {
		t.Errorf("CidrToLong end = %d, want 3232236031", result[1])
	}
}
