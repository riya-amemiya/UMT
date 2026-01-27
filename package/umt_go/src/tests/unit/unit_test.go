package unit_test

import (
	"math"
	"testing"

	"github.com/riya-amemiya/umt-go/src/unit"
)

func TestNewUnitConverterBasic(t *testing.T) {
	converter := unit.NewUnitConverter(map[string]float64{
		"meters":      1,
		"kilometers":  0.001,
		"centimeters": 100,
		"millimeters": 1000,
	})

	tests := []struct {
		name     string
		value    float64
		from, to string
		expected float64
	}{
		{"meters to km", 1000, "meters", "kilometers", 1.0},
		{"km to meters", 1, "kilometers", "meters", 1000.0},
		{"meters to cm", 1, "meters", "centimeters", 100.0},
		{"cm to meters", 100, "centimeters", "meters", 1.0},
		{"meters to mm", 1, "meters", "millimeters", 1000.0},
		{"km to cm", 1, "kilometers", "centimeters", 100000.0},
		{"same unit", 42, "meters", "meters", 42.0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := converter.Convert(tt.value, tt.from, tt.to)
			if math.Abs(result-tt.expected) > 1e-10 {
				t.Errorf("Convert(%v, %q, %q) = %v, want %v",
					tt.value, tt.from, tt.to, result, tt.expected)
			}
		})
	}
}

func TestNewUnitConverterPanics(t *testing.T) {
	converter := unit.NewUnitConverter(map[string]float64{
		"meters":     1,
		"kilometers": 0.001,
	})

	t.Run("unknown from unit", func(t *testing.T) {
		defer func() {
			if r := recover(); r == nil {
				t.Error("Expected panic for unknown from unit")
			}
		}()
		converter.Convert(1, "feet", "meters")
	})

	t.Run("unknown to unit", func(t *testing.T) {
		defer func() {
			if r := recover(); r == nil {
				t.Error("Expected panic for unknown to unit")
			}
		}()
		converter.Convert(1, "meters", "feet")
	})
}

func TestUnitConverterInitialization(t *testing.T) {
	convertLength := unit.UnitConverterInitialization(map[string]float64{
		"meters":      1,
		"kilometers":  0.001,
		"centimeters": 100,
		"millimeters": 1000,
	})

	tests := []struct {
		name     string
		value    float64
		from, to string
		expected float64
	}{
		{"meters to km", 1000, "meters", "kilometers", 1.0},
		{"km to meters", 1, "kilometers", "meters", 1000.0},
		{"meters to cm", 1, "meters", "centimeters", 100.0},
		{"cm to mm", 1, "centimeters", "millimeters", 10.0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := convertLength(tt.value, tt.from, tt.to)
			if math.Abs(result-tt.expected) > 1e-10 {
				t.Errorf("convertLength(%v, %q, %q) = %v, want %v",
					tt.value, tt.from, tt.to, result, tt.expected)
			}
		})
	}
}

func TestUnitConverterWeight(t *testing.T) {
	convertWeight := unit.UnitConverterInitialization(map[string]float64{
		"grams":     1,
		"kilograms": 0.001,
		"milligrams": 1000,
		"pounds":    0.00220462,
	})

	result := convertWeight(1, "kilograms", "grams")
	if math.Abs(result-1000) > 1e-6 {
		t.Errorf("1 kg to grams = %v, want 1000", result)
	}

	result = convertWeight(1000, "grams", "kilograms")
	if math.Abs(result-1) > 1e-6 {
		t.Errorf("1000 grams to kg = %v, want 1", result)
	}
}
