package math

import (
	"testing"
)

func TestIsPrimeNumber(t *testing.T) {
	tests := []struct {
		name     string
		n        int
		expected bool
	}{
		{"2 is prime", 2, true},
		{"3 is prime", 3, true},
		{"5 is prime", 5, true},
		{"7 is prime", 7, true},
		{"11 is prime", 11, true},
		{"13 is prime", 13, true},
		{"17 is prime", 17, true},
		{"31 is prime", 31, true},
		{"997 is prime", 997, true},
		{"4 is not prime", 4, false},
		{"6 is not prime", 6, false},
		{"8 is not prime", 8, false},
		{"9 is not prime", 9, false},
		{"10 is not prime", 10, false},
		{"15 is not prime", 15, false},
		{"100 is not prime", 100, false},
		{"0 is not prime", 0, false},
		{"1 is not prime", 1, false},
		{"-1 is not prime", -1, false},
		{"-5 is not prime", -5, false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := IsPrimeNumber(tt.n)
			if result != tt.expected {
				t.Errorf("IsPrimeNumber(%d) = %v, want %v", tt.n, result, tt.expected)
			}
		})
	}
}

func TestPrimeFactorization(t *testing.T) {
	tests := []struct {
		name     string
		n        int
		expected []int
	}{
		{"factorize 12", 12, []int{2, 2, 3}},
		{"factorize 14", 14, []int{2, 7}},
		{"factorize 15", 15, []int{3, 5}},
		{"factorize 18", 18, []int{2, 3, 3}},
		{"factorize 20", 20, []int{2, 2, 5}},
		{"factorize 16 (powers)", 16, []int{2, 2, 2, 2}},
		{"factorize 27 (powers)", 27, []int{3, 3, 3}},
		{"factorize 32 (powers)", 32, []int{2, 2, 2, 2, 2}},
		{"factorize prime 17", 17, []int{17}},
		{"factorize prime 19", 19, []int{19}},
		{"factorize prime 23", 23, []int{23}},
		{"factorize 0", 0, []int{}},
		{"factorize 1", 1, []int{}},
		{"factorize -12", -12, []int{2, 2, 3}},
		{"factorize -15", -15, []int{3, 5}},
		{"factorize large prime 997", 997, []int{997}},
		{"factorize 1000", 1000, []int{2, 2, 2, 5, 5, 5}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := PrimeFactorization(tt.n)
			if len(result) != len(tt.expected) {
				t.Errorf("PrimeFactorization(%d) = %v, want %v", tt.n, result, tt.expected)
				return
			}
			for i := range result {
				if result[i] != tt.expected[i] {
					t.Errorf("PrimeFactorization(%d) = %v, want %v", tt.n, result, tt.expected)
					return
				}
			}
		})
	}
}

func TestFactorize(t *testing.T) {
	tests := []struct {
		name     string
		n        int
		expected []int
	}{
		{"factorize 12", 12, []int{1, 2, 3, 4, 6, 12}},
		{"factorize 7 (prime)", 7, []int{1, 7}},
		{"factorize 1", 1, []int{1}},
		{"factorize 0", 0, []int{}},
		{"factorize 25", 25, []int{1, 5, 25}},
		{"factorize 36", 36, []int{1, 2, 3, 4, 6, 9, 12, 18, 36}},
		{"factorize 100", 100, []int{1, 2, 4, 5, 10, 20, 25, 50, 100}},
		{"factorize negative", -12, []int{1, 2, 3, 4, 6, 12}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := Factorize(tt.n)
			if len(result) != len(tt.expected) {
				t.Errorf("Factorize(%d) = %v, want %v", tt.n, result, tt.expected)
				return
			}
			for i := range result {
				if result[i] != tt.expected[i] {
					t.Errorf("Factorize(%d) = %v, want %v", tt.n, result, tt.expected)
					return
				}
			}
		})
	}
}

func TestGCD(t *testing.T) {
	tests := []struct {
		name     string
		a, b     int
		expected int
	}{
		{"56 and 48", 56, 48, 8},
		{"12 and 18", 12, 18, 6},
		{"48 and 18", 48, 18, 6},
		{"56 and 0", 56, 0, 56},
		{"0 and 56", 0, 56, 56},
		{"0 and 0", 0, 0, 0},
		{"56 and 1", 56, 1, 1},
		{"1 and 56", 1, 56, 1},
		{"1 and 1", 1, 1, 1},
		{"-56 and 48", -56, 48, 8},
		{"56 and -48", 56, -48, 8},
		{"-56 and -48", -56, -48, 8},
		{"1000000 and 500000", 1000000, 500000, 500000},
		{"1 and 2", 1, 2, 1},
		{"2 and 3", 2, 3, 1},
		{"42 and 42", 42, 42, 42},
		{"2 and 4", 2, 4, 2},
		{"3 and 9", 3, 9, 3},
		{"5 and 25", 5, 25, 5},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := GCD(tt.a, tt.b)
			if result != tt.expected {
				t.Errorf("GCD(%d, %d) = %d, want %d", tt.a, tt.b, result, tt.expected)
			}
		})
	}
}

func TestLCM(t *testing.T) {
	tests := []struct {
		name     string
		a, b     int
		expected int
	}{
		{"4 and 5", 4, 5, 20},
		{"6 and 8", 6, 8, 24},
		{"10 and 15", 10, 15, 30},
		{"7 and 11", 7, 11, 77},
		{"2 and 3", 2, 3, 6},
		{"12 and 18", 12, 18, 36},
		{"0 and 5", 0, 5, 0},
		{"4 and 0", 4, 0, 0},
		{"0 and 0", 0, 0, 0},
		{"1 and 5", 1, 5, 5},
		{"4 and 1", 4, 1, 4},
		{"1 and 1", 1, 1, 1},
		{"1000 and 2000", 1000, 2000, 2000},
		{"999 and 1001", 999, 1001, 999999},
		{"-4 and 6", -4, 6, 12},
		{"4 and -6", 4, -6, 12},
		{"-4 and -6", -4, -6, 12},
		{"5 and 5", 5, 5, 5},
		{"42 and 42", 42, 42, 42},
		{"3 and 5", 3, 5, 15},
		{"13 and 17", 13, 17, 221},
		{"6 and 12", 6, 12, 12},
		{"5 and 25", 5, 25, 25},
		{"4 and 20", 4, 20, 20},
		{"2 and 2", 2, 2, 2},
		{"100 and 200", 100, 200, 200},
		{"17 and 1", 17, 1, 17},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := LCM(tt.a, tt.b)
			if result != tt.expected {
				t.Errorf("LCM(%d, %d) = %d, want %d", tt.a, tt.b, result, tt.expected)
			}
		})
	}
}
