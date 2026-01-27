package math

// ToCelsius converts temperature from Kelvin to Celsius.
//
// Example:
//
//	ToCelsius(273.15) // 0
//	ToCelsius(300)    // 26.85
//	ToCelsius(0)      // -273.15 (absolute zero)
func ToCelsius(kelvin float64) float64 {
	return Subtraction(kelvin, 273.15)
}

// ToKelvin converts temperature from Celsius to Kelvin.
//
// Example:
//
//	ToKelvin(0)      // 273.15
//	ToKelvin(26.85)  // 300
//	ToKelvin(-273.15) // 0 (absolute zero)
func ToKelvin(celsius float64) float64 {
	return Addition(celsius, 273.15)
}
