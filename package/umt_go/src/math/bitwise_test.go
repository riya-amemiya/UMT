package math

import (
	"testing"
)

func TestBitwiseAnd(t *testing.T) {
	tests := []struct {
		name     string
		a, b     int
		expected int
	}{
		{"5 AND 3", 5, 3, 1},
		{"0xFF AND 0x0F", 0xFF, 0x0F, 0x0F},
		{"12 AND 10", 12, 10, 8},
		{"0 AND 0", 0, 0, 0},
		{"0 AND 5", 0, 5, 0},
		{"-1 AND 5", -1, 5, 5},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := BitwiseAnd(tt.a, tt.b)
			if result != tt.expected {
				t.Errorf("BitwiseAnd(%d, %d) = %d, want %d", tt.a, tt.b, result, tt.expected)
			}
		})
	}
}

func TestBitwiseOr(t *testing.T) {
	tests := []struct {
		name     string
		a, b     int
		expected int
	}{
		{"5 OR 3", 5, 3, 7},
		{"0xF0 OR 0x0F", 0xF0, 0x0F, 0xFF},
		{"12 OR 10", 12, 10, 14},
		{"0 OR 0", 0, 0, 0},
		{"0 OR 5", 0, 5, 5},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := BitwiseOr(tt.a, tt.b)
			if result != tt.expected {
				t.Errorf("BitwiseOr(%d, %d) = %d, want %d", tt.a, tt.b, result, tt.expected)
			}
		})
	}
}

func TestBitwiseXor(t *testing.T) {
	tests := []struct {
		name     string
		a, b     int
		expected int
	}{
		{"5 XOR 3", 5, 3, 6},
		{"0xFF XOR 0x0F", 0xFF, 0x0F, 0xF0},
		{"12 XOR 10", 12, 10, 6},
		{"0 XOR 0", 0, 0, 0},
		{"5 XOR 0", 5, 0, 5},
		{"same values", 42, 42, 0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := BitwiseXor(tt.a, tt.b)
			if result != tt.expected {
				t.Errorf("BitwiseXor(%d, %d) = %d, want %d", tt.a, tt.b, result, tt.expected)
			}
		})
	}
}

func TestBitwiseNot(t *testing.T) {
	tests := []struct {
		name     string
		a        int
		expected int
	}{
		{"NOT 0", 0, -1},
		{"NOT 5", 5, -6},
		{"NOT -1", -1, 0},
		{"NOT 1", 1, -2},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := BitwiseNot(tt.a)
			if result != tt.expected {
				t.Errorf("BitwiseNot(%d) = %d, want %d", tt.a, result, tt.expected)
			}
		})
	}
}

func TestBitwiseLeftShift(t *testing.T) {
	tests := []struct {
		name     string
		a, b     int
		expected int
	}{
		{"1 << 0", 1, 0, 1},
		{"1 << 1", 1, 1, 2},
		{"1 << 3", 1, 3, 8},
		{"5 << 2", 5, 2, 20},
		{"0 << 5", 0, 5, 0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := BitwiseLeftShift(tt.a, tt.b)
			if result != tt.expected {
				t.Errorf("BitwiseLeftShift(%d, %d) = %d, want %d", tt.a, tt.b, result, tt.expected)
			}
		})
	}
}

func TestBitwiseRightShift(t *testing.T) {
	tests := []struct {
		name     string
		a, b     int
		expected int
	}{
		{"8 >> 0", 8, 0, 8},
		{"8 >> 1", 8, 1, 4},
		{"8 >> 3", 8, 3, 1},
		{"20 >> 2", 20, 2, 5},
		{"0 >> 5", 0, 5, 0},
		{"1 >> 1", 1, 1, 0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := BitwiseRightShift(tt.a, tt.b)
			if result != tt.expected {
				t.Errorf("BitwiseRightShift(%d, %d) = %d, want %d", tt.a, tt.b, result, tt.expected)
			}
		})
	}
}
