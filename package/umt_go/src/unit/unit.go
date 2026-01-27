package unit

// UnitConverter holds a set of conversion ratios relative to a base unit
// and provides a method to convert values between any two defined units.
//
// Each ratio represents how many of that unit equals the base unit amount.
// For example, for length with meters as the base unit:
//
//	ratios = {"meters": 1, "kilometers": 0.001, "centimeters": 100, "millimeters": 1000}
//
// This means 1 meter = 0.001 kilometers, 1 meter = 100 centimeters, etc.
type UnitConverter struct {
	ratios map[string]float64
}

// NewUnitConverter creates a new UnitConverter with the given unit-to-base-unit ratios.
//
// The ratios map defines conversion factors where each key is a unit name and
// each value represents how many of that unit equals one base unit.
//
// Example:
//
//	converter := NewUnitConverter(map[string]float64{
//	    "meters":      1,     // base unit
//	    "kilometers":  0.001,
//	    "centimeters": 100,
//	    "millimeters": 1000,
//	})
func NewUnitConverter(ratios map[string]float64) *UnitConverter {
	// Copy the ratios map to prevent external modification
	copied := make(map[string]float64, len(ratios))
	for k, v := range ratios {
		copied[k] = v
	}
	return &UnitConverter{ratios: copied}
}

// Convert converts a value from one unit to another.
//
// The conversion formula is: result = (value / fromRatio) * toRatio
// This first converts the value to the base unit, then to the target unit.
//
// Panics if either the from or to unit is not defined in the converter's ratios.
//
// Example:
//
//	converter := NewUnitConverter(map[string]float64{
//	    "meters":      1,
//	    "kilometers":  0.001,
//	    "centimeters": 100,
//	})
//	converter.Convert(1000, "meters", "kilometers")   // 1.0
//	converter.Convert(1, "kilometers", "meters")       // 1000.0
//	converter.Convert(1, "meters", "centimeters")      // 100.0
func (uc *UnitConverter) Convert(value float64, from, to string) float64 {
	fromRatio, ok := uc.ratios[from]
	if !ok {
		panic("unknown unit: " + from)
	}
	toRatio, ok := uc.ratios[to]
	if !ok {
		panic("unknown unit: " + to)
	}
	return (value / fromRatio) * toRatio
}

// UnitConverterInitialization creates a converter function from a map of unit ratios.
// This is a functional-style API that mirrors the TypeScript implementation.
//
// The returned function converts a value from one unit to another using the
// formula: result = (value / fromRatio) * toRatio
//
// Example:
//
//	convertLength := UnitConverterInitialization(map[string]float64{
//	    "meters":      1,
//	    "kilometers":  0.001,
//	    "centimeters": 100,
//	    "millimeters": 1000,
//	})
//	convertLength(1000, "meters", "kilometers") // 1.0
//	convertLength(1, "kilometers", "meters")    // 1000.0
func UnitConverterInitialization(ratios map[string]float64) func(value float64, from, to string) float64 {
	// Copy the ratios map to prevent external modification
	copied := make(map[string]float64, len(ratios))
	for k, v := range ratios {
		copied[k] = v
	}
	return func(value float64, from, to string) float64 {
		fromRatio := copied[from]
		toRatio := copied[to]
		return (value / fromRatio) * toRatio
	}
}
