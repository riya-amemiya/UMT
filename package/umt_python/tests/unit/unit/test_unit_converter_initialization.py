import unittest

from src.unit import unit_converter_initialization


class TestUnitConverterInitialization(unittest.TestCase):
    def test_basic_conversion(self):
        """Test basic unit conversion.

        The formula is: (value / from_unit_ratio) * to_unit_ratio
        With meters=1, kilometers=1000, centimeters=0.01:
        - 5 km to m: (5 / 1000) * 1 = 0.005
        - 1 m to cm: (1 / 1) * 0.01 = 0.01
        """
        length_converter = unit_converter_initialization(
            {
                "meters": 1,
                "kilometers": 1000,
                "centimeters": 0.01,
            }
        )

        self.assertEqual(length_converter(5, "kilometers", "meters"), 0.005)

        self.assertEqual(length_converter(1, "meters", "centimeters"), 0.01)

        self.assertAlmostEqual(
            length_converter(100000, "centimeters", "kilometers"), 10000000000.0
        )

    def test_conversion_formula(self):
        """Test the actual conversion formula: (value / from_ratio) * to_ratio."""
        converter = unit_converter_initialization(
            {
                "a": 2,
                "b": 4,
            }
        )

        self.assertEqual(converter(10, "a", "b"), 20.0)

        self.assertEqual(converter(10, "b", "a"), 5.0)

    def test_same_unit_conversion(self):
        """Test conversion between the same unit."""
        converter = unit_converter_initialization(
            {
                "a": 1,
                "b": 2,
            }
        )
        self.assertEqual(converter(10, "a", "a"), 10.0)
        self.assertEqual(converter(10, "b", "b"), 10.0)

    def test_weight_conversion(self):
        """Test weight conversion as another use case."""
        weight_converter = unit_converter_initialization(
            {
                "grams": 1,
                "kilograms": 1000,
                "milligrams": 0.001,
            }
        )

        self.assertEqual(weight_converter(2, "kilograms", "grams"), 0.002)

        self.assertEqual(weight_converter(1, "grams", "milligrams"), 0.001)

    def test_fractional_values(self):
        """Test conversion with fractional values."""
        converter = unit_converter_initialization(
            {
                "base": 1,
                "double": 2,
                "half": 0.5,
            }
        )

        self.assertEqual(converter(0.5, "double", "base"), 0.25)

        self.assertEqual(converter(2, "base", "half"), 1.0)


if __name__ == "__main__":
    unittest.main()
