package math

import (
	stdmath "math"
	"testing"
)

func TestDegToRad(t *testing.T) {
	tests := []struct {
		name     string
		deg      float64
		expected float64
	}{
		{"0 degrees", 0, 0},
		{"90 degrees", 90, stdmath.Pi / 2},
		{"180 degrees", 180, stdmath.Pi},
		{"270 degrees", 270, 3 * stdmath.Pi / 2},
		{"360 degrees", 360, 2 * stdmath.Pi},
		{"-90 degrees", -90, -stdmath.Pi / 2},
		{"-180 degrees", -180, -stdmath.Pi},
		{"-270 degrees", -270, -3 * stdmath.Pi / 2},
		{"-360 degrees", -360, -2 * stdmath.Pi},
		{"45.5 degrees", 45.5, stdmath.Pi/4 + stdmath.Pi/360},
		{"450 degrees", 450, 5 * stdmath.Pi / 2},
		{"720 degrees", 720, 4 * stdmath.Pi},
		{"1080 degrees", 1080, 6 * stdmath.Pi},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := DegToRad(tt.deg)
			if stdmath.Abs(result-tt.expected) > 1e-10 {
				t.Errorf("DegToRad(%v) = %v, want close to %v", tt.deg, result, tt.expected)
			}
		})
	}

	t.Run("NaN", func(t *testing.T) {
		if !stdmath.IsNaN(DegToRad(stdmath.NaN())) {
			t.Error("DegToRad(NaN) should return NaN")
		}
	})

	t.Run("positive infinity", func(t *testing.T) {
		if !stdmath.IsInf(DegToRad(stdmath.Inf(1)), 1) {
			t.Error("DegToRad(+Inf) should return +Inf")
		}
	})

	t.Run("negative infinity", func(t *testing.T) {
		if !stdmath.IsInf(DegToRad(stdmath.Inf(-1)), -1) {
			t.Error("DegToRad(-Inf) should return -Inf")
		}
	})
}

func TestRadToDeg(t *testing.T) {
	tests := []struct {
		name     string
		rad      float64
		expected float64
	}{
		{"0 radians", 0, 0},
		{"Pi/2", stdmath.Pi / 2, 90},
		{"Pi", stdmath.Pi, 180},
		{"3Pi/2", 3 * stdmath.Pi / 2, 270},
		{"2Pi", 2 * stdmath.Pi, 360},
		{"-Pi/2", -stdmath.Pi / 2, -90},
		{"-Pi", -stdmath.Pi, -180},
		{"-3Pi/2", -3 * stdmath.Pi / 2, -270},
		{"-2Pi", -2 * stdmath.Pi, -360},
		{"Pi/4", stdmath.Pi / 4, 45},
		{"5Pi/2", 5 * stdmath.Pi / 2, 450},
		{"4Pi", 4 * stdmath.Pi, 720},
		{"6Pi", 6 * stdmath.Pi, 1080},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := RadToDeg(tt.rad)
			if stdmath.Abs(result-tt.expected) > 1e-10 {
				t.Errorf("RadToDeg(%v) = %v, want close to %v", tt.rad, result, tt.expected)
			}
		})
	}

	t.Run("NaN", func(t *testing.T) {
		if !stdmath.IsNaN(RadToDeg(stdmath.NaN())) {
			t.Error("RadToDeg(NaN) should return NaN")
		}
	})

	t.Run("positive infinity", func(t *testing.T) {
		if !stdmath.IsInf(RadToDeg(stdmath.Inf(1)), 1) {
			t.Error("RadToDeg(+Inf) should return +Inf")
		}
	})

	t.Run("negative infinity", func(t *testing.T) {
		if !stdmath.IsInf(RadToDeg(stdmath.Inf(-1)), -1) {
			t.Error("RadToDeg(-Inf) should return -Inf")
		}
	})
}

func TestToBaseN(t *testing.T) {
	tests := []struct {
		name     string
		n        int
		base     int
		expected string
	}{
		{"1 to binary", 1, 2, "1"},
		{"2 to binary", 2, 2, "10"},
		{"3 to binary", 3, 2, "11"},
		{"4 to binary", 4, 2, "100"},
		{"5 to binary", 5, 2, "101"},
		{"6 to binary", 6, 2, "110"},
		{"112 to binary", 112, 2, "1110000"},
		{"7 to base 4", 7, 4, "13"},
		{"8 to base 4", 8, 4, "20"},
		{"9 to base 4", 9, 4, "21"},
		{"112 to base 4", 112, 4, "1300"},
		{"7 to octal", 7, 8, "7"},
		{"8 to octal", 8, 8, "10"},
		{"9 to octal", 9, 8, "11"},
		{"112 to octal", 112, 8, "160"},
		{"10 to hex", 10, 16, "a"},
		{"15 to hex", 15, 16, "f"},
		{"16 to hex", 16, 16, "10"},
		{"255 to hex", 255, 16, "ff"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := ToBaseN(tt.n, tt.base)
			if result != tt.expected {
				t.Errorf("ToBaseN(%d, %d) = %q, want %q", tt.n, tt.base, result, tt.expected)
			}
		})
	}
}

func TestFlexibleNumberConversion(t *testing.T) {
	tests := []struct {
		name     string
		value    string
		fromBase int
		toBase   int
		expected string
		hasError bool
	}{
		{"hex to decimal", "ff", 16, 10, "255", false},
		{"binary to decimal", "1010", 2, 10, "10", false},
		{"decimal to hex", "255", 10, 16, "ff", false},
		{"decimal to binary", "10", 10, 2, "1010", false},
		{"octal to decimal", "77", 8, 10, "63", false},
		{"decimal to octal", "63", 10, 8, "77", false},
		{"hex to binary", "ff", 16, 2, "11111111", false},
		{"binary to hex", "11111111", 2, 16, "ff", false},
		{"invalid input", "xyz", 10, 16, "", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := FlexibleNumberConversion(tt.value, tt.fromBase, tt.toBase)
			if tt.hasError {
				if err == nil {
					t.Errorf("FlexibleNumberConversion(%q, %d, %d) expected error, got %q", tt.value, tt.fromBase, tt.toBase, result)
				}
				return
			}
			if err != nil {
				t.Errorf("FlexibleNumberConversion(%q, %d, %d) unexpected error: %v", tt.value, tt.fromBase, tt.toBase, err)
				return
			}
			if result != tt.expected {
				t.Errorf("FlexibleNumberConversion(%q, %d, %d) = %q, want %q", tt.value, tt.fromBase, tt.toBase, result, tt.expected)
			}
		})
	}
}

func TestGetDecimalLength(t *testing.T) {
	tests := []struct {
		name     string
		n        float64
		expected int
	}{
		{"integer 1", 1, 0},
		{"integer 100", 100, 0},
		{"negative integer", -42, 0},
		{"1.0 (no decimals)", 1.0, 0},
		{"zero", 0, 0},
		{"-0", -0.0, 0},
		{"1.1", 1.1, 1},
		{"1.11", 1.11, 2},
		{"1.123", 1.123, 3},
		{"1.01", 1.01, 2},
		{"1.001", 1.001, 3},
		{"1.000001", 1.000001, 6},
		{"-1.1", -1.1, 1},
		{"-0.01", -0.01, 2},
		{"123456.789", 123456.789, 3},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := GetDecimalLength(tt.n)
			if result != tt.expected {
				t.Errorf("GetDecimalLength(%v) = %d, want %d", tt.n, result, tt.expected)
			}
		})
	}
}

func TestRoundOf(t *testing.T) {
	tests := []struct {
		name      string
		n         float64
		precision int
		expected  float64
	}{
		{"round to 0 places (down)", 1.111111111111, 0, 1},
		{"round to 0 places (up)", 1.555555555555, 0, 2},
		{"round to 0 places (boundary)", 1.499999999999, 0, 1},
		{"round to 2 places (down)", 1.111111111111, 2, 1.11},
		{"round to 2 places (up)", 1.555555555555, 2, 1.56},
		{"round to 2 places (boundary)", 1.499999999999, 2, 1.5},
		{"round to 1 place", 1.111111111111, 1, 1.1},
		{"round to 3 places", 1.111111111111, 3, 1.111},
		{"round to 4 places", 1.111111111111, 4, 1.1111},
		{"round to 5 places", 1.111111111111, 5, 1.11111},
		{"negative number down", -1.234, 2, -1.23},
		{"negative number up", -1.235, 2, -1.24},
		{"zero precision 0", 0, 0, 0},
		{"zero precision 2", 0, 2, 0},
		{"negative precision -1", 1234.5678, -1, 1230},
		{"negative precision -2", 1234.5678, -2, 1200},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := RoundOf(tt.n, tt.precision)
			if stdmath.Abs(result-tt.expected) > 1e-10 {
				t.Errorf("RoundOf(%v, %d) = %v, want %v", tt.n, tt.precision, result, tt.expected)
			}
		})
	}
}

func TestMathSeparator(t *testing.T) {
	tests := []struct {
		name     string
		n        float64
		sep      string
		expected string
	}{
		{"thousands", 1234567.89, ",", "1,234,567.89"},
		{"simple thousands", 1000, ",", "1,000"},
		{"no separator needed", 100, ",", "100"},
		{"zero", 0, ",", "0"},
		{"negative", -1234567.89, ",", "-1,234,567.89"},
		{"single digit", 5, ",", "5"},
		{"with different separator", 1234567, ".", "1.234.567"},
		{"millions", 1000000, ",", "1,000,000"},
		{"just decimal", 0.123, ",", "0.123"},
		{"ten thousand", 10000, ",", "10,000"},
		{"hundred thousand", 100000, ",", "100,000"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := MathSeparator(tt.n, tt.sep)
			if result != tt.expected {
				t.Errorf("MathSeparator(%v, %q) = %q, want %q", tt.n, tt.sep, result, tt.expected)
			}
		})
	}
}
