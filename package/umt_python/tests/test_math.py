import math
import unittest

from src.math import (
    addition,
    average,
    bitwise,
    correlation_coefficient,
    deg_to_rad,
    deviation_value,
    division,
    factorial,
    factorize,
    gcd,
    get_decimal_length,
    is_double,
    is_prime_number,
    lcm,
    max_value,
    median,
    min_value,
    mode,
    multiples,
    multiplication,
    percentile,
    prime_factorization,
    quotient,
    rad_to_deg,
    reduce_fraction,
    round_of,
    standard_deviation,
    subtraction,
    to_base_n,
)
from src import to_celsius, to_kelvin


class TestAddition(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(addition(2, 3), 5)
        self.assertEqual(addition(-2, -3), -5)
        self.assertEqual(addition(2, -3), -1)
        self.assertEqual(addition(1, 2, 3), 6)
        self.assertEqual(addition(-1, -2, -3), -6)
        self.assertEqual(addition(2, -3, 1), 0)

    def test_floating_point(self):
        self.assertAlmostEqual(addition(0.1, 0.2), 0.3)
        self.assertAlmostEqual(addition(-0.1, -0.2), -0.3)
        self.assertAlmostEqual(addition(0.1, -0.2), -0.1)
        self.assertAlmostEqual(addition(2, 0.3), 2.3)
        self.assertAlmostEqual(addition(-2, -0.3), -2.3)
        self.assertAlmostEqual(addition(2, -0.3), 1.7)
        self.assertAlmostEqual(addition(-2, 0.3), -1.7)
        self.assertAlmostEqual(addition(0.1, 0.2, 0.3), 0.6)
        self.assertAlmostEqual(addition(-2, 0.5, 1.5), 0)

    def test_docstring_example(self):
        self.assertAlmostEqual(addition(0.1, 0.2), 0.3)
        self.assertEqual(addition(1, 2, 3), 6)


class TestSubtraction(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(subtraction(5, 3), 2)
        self.assertEqual(subtraction(-5, -3), -2)
        self.assertEqual(subtraction(5, -3), 8)

    def test_floating_point(self):
        self.assertAlmostEqual(subtraction(0.3, 0.1), 0.2)
        self.assertAlmostEqual(subtraction(1, 0.1, 0.2), 0.7)

    def test_docstring_example(self):
        self.assertAlmostEqual(subtraction(0.3, 0.1), 0.2)
        self.assertAlmostEqual(subtraction(1, 0.1, 0.2), 0.7)


class TestMultiplication(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(multiplication(2, 3), 6)
        self.assertEqual(multiplication(2, 3, 4), 24)
        self.assertEqual(multiplication(-2, 3), -6)

    def test_floating_point(self):
        self.assertAlmostEqual(multiplication(0.1, 0.2), 0.02)
        self.assertAlmostEqual(multiplication(0.1, 0.2, 0.3), 0.006)

    def test_docstring_example(self):
        self.assertAlmostEqual(multiplication(0.1, 0.2, 0.3), 0.006)
        self.assertEqual(multiplication(2, 3, 4), 24)


class TestDivision(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(division(6, 3), 2.0)
        self.assertAlmostEqual(division(0.1, 0.2), 0.5)

    def test_with_remainder(self):
        result = division(10, 3, False)
        self.assertEqual(result[0], 3)
        self.assertEqual(result[1], 1)

    def test_division_by_zero(self):
        self.assertTrue(math.isnan(division(1, 0)))
        result = division(1, 0, False)
        self.assertTrue(math.isnan(result[0]))
        self.assertTrue(math.isnan(result[1]))

    def test_docstring_example(self):
        self.assertAlmostEqual(division(0.1, 0.2), 0.5)
        result = division(10, 3, False)
        self.assertEqual(result[0], 3)
        self.assertEqual(result[1], 1)


class TestAverage(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(average([1, 2, 3]), 2)
        self.assertEqual(average([10, 20]), 15)

    def test_edge_cases(self):
        self.assertEqual(average([]), 0)
        self.assertEqual(average([5]), 5)

    def test_docstring_example(self):
        self.assertEqual(average([1, 2, 3]), 2)
        self.assertEqual(average([10, 20]), 15)
        self.assertEqual(average([]), 0)


class TestMedian(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(median([1, 3, 3, 6, 7, 8, 9]), 6)
        self.assertEqual(median([1, 2, 3, 4]), 2.5)

    def test_edge_cases(self):
        self.assertEqual(median([5]), 5)
        self.assertEqual(median([1, 2]), 1.5)

    def test_docstring_example(self):
        self.assertEqual(median([1, 3, 3, 6, 7, 8, 9]), 6)
        self.assertEqual(median([1, 2, 3, 4]), 2.5)


class TestMode(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(mode([1, 2, 2, 3, 3, 3]), [3])
        self.assertEqual(mode([1, 2, 2, 3, 3]), [2, 3])
        self.assertEqual(mode([1, 2, 3]), [1, 2, 3])

    def test_edge_cases(self):
        self.assertEqual(mode([]), [])
        self.assertEqual(mode([5]), [5])

    def test_docstring_example(self):
        self.assertEqual(mode([1, 2, 2, 3, 3, 3]), [3])
        self.assertEqual(mode([1, 2, 2, 3, 3]), [2, 3])
        self.assertEqual(mode([1, 2, 3]), [1, 2, 3])


class TestStandardDeviation(unittest.TestCase):
    def test_basic_cases(self):
        self.assertAlmostEqual(standard_deviation([1, 2, 3]), 0.816497, places=6)
        self.assertAlmostEqual(
            standard_deviation([10, 12, 23, 23, 16, 23, 21, 16]), 4.89898, places=5
        )

    def test_edge_cases(self):
        self.assertEqual(standard_deviation([5, 5, 5]), 0)

    def test_docstring_example(self):
        self.assertAlmostEqual(
            round(standard_deviation([1, 2, 3]), 6), 0.816497, places=6
        )


class TestFactorial(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(factorial(5), 120)
        self.assertEqual(factorial(0), 1)
        self.assertEqual(factorial(1), 1)

    def test_edge_cases(self):
        self.assertEqual(factorial(10), 3628800)
        self.assertEqual(factorial(-1), 1)

    def test_docstring_example(self):
        self.assertEqual(factorial(5), 120)
        self.assertEqual(factorial(0), 1)


class TestGcd(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(gcd(12, 18), 6)
        self.assertEqual(gcd(12, 18, 24), 6)

    def test_floating_point(self):
        self.assertEqual(gcd(0.5, 0.25), 0.25)

    def test_edge_cases(self):
        self.assertEqual(gcd(0, 5), 5)
        self.assertEqual(gcd(5, 0), 5)

    def test_docstring_example(self):
        self.assertEqual(gcd(12, 18), 6)
        self.assertEqual(gcd(12, 18, 24), 6)
        self.assertEqual(gcd(0.5, 0.25), 0.25)


class TestLcm(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(lcm(2, 3), 6)
        self.assertEqual(lcm(4, 6), 12)

    def test_edge_cases(self):
        self.assertEqual(lcm(0, 5), 0)
        self.assertEqual(lcm(5, 0), 0)

    def test_docstring_example(self):
        self.assertEqual(lcm(2, 3), 6)
        self.assertEqual(lcm(4, 6), 12)


class TestIsPrimeNumber(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_prime_number(2))
        self.assertTrue(is_prime_number(17))
        self.assertFalse(is_prime_number(4))
        self.assertFalse(is_prime_number(1))
        self.assertFalse(is_prime_number(-3))

    def test_edge_cases(self):
        self.assertFalse(is_prime_number(0))
        self.assertTrue(is_prime_number(3))
        self.assertTrue(is_prime_number(97))

    def test_docstring_example(self):
        self.assertTrue(is_prime_number(2))
        self.assertTrue(is_prime_number(17))
        self.assertFalse(is_prime_number(4))
        self.assertFalse(is_prime_number(1))
        self.assertFalse(is_prime_number(-3))


class TestPrimeFactorization(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(
            prime_factorization(12),
            [{"number": 2, "count": 2}, {"number": 3, "count": 1}],
        )
        self.assertEqual(prime_factorization(7), [{"number": 7, "count": 1}])

    def test_edge_cases(self):
        self.assertEqual(
            prime_factorization(100),
            [{"number": 2, "count": 2}, {"number": 5, "count": 2}],
        )

    def test_docstring_example(self):
        self.assertEqual(
            prime_factorization(12),
            [{"number": 2, "count": 2}, {"number": 3, "count": 1}],
        )


class TestDegToRad(unittest.TestCase):
    def test_basic_cases(self):
        self.assertAlmostEqual(deg_to_rad(180), math.pi)
        self.assertAlmostEqual(deg_to_rad(90), math.pi / 2)
        self.assertAlmostEqual(deg_to_rad(0), 0)

    def test_edge_cases(self):
        self.assertAlmostEqual(deg_to_rad(360), 2 * math.pi)
        self.assertAlmostEqual(deg_to_rad(-90), -math.pi / 2)

    def test_docstring_example(self):
        self.assertAlmostEqual(round(deg_to_rad(180), 10), 3.1415926536)


class TestRadToDeg(unittest.TestCase):
    def test_basic_cases(self):
        self.assertAlmostEqual(rad_to_deg(math.pi), 180)
        self.assertAlmostEqual(rad_to_deg(math.pi / 2), 90)
        self.assertAlmostEqual(rad_to_deg(0), 0)

    def test_edge_cases(self):
        self.assertAlmostEqual(rad_to_deg(2 * math.pi), 360)
        self.assertAlmostEqual(rad_to_deg(-math.pi / 2), -90)

    def test_docstring_example(self):
        self.assertEqual(rad_to_deg(math.pi), 180.0)


class TestRoundOf(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(round_of(1.234, 2), 1.23)
        self.assertEqual(round_of(1.235, 2), 1.24)
        self.assertEqual(round_of(-1.234, 2), -1.23)

    def test_edge_cases(self):
        self.assertEqual(round_of(1.5, 0), 2)
        self.assertEqual(round_of(1.4, 0), 1)

    def test_docstring_example(self):
        self.assertEqual(round_of(1.234, 2), 1.23)
        self.assertEqual(round_of(1.235, 2), 1.24)
        self.assertEqual(round_of(-1.234, 2), -1.23)


class TestMaxValue(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(max_value(1, 2, 3), 3)
        self.assertEqual(max_value(-1, -2, -3), -1)

    def test_edge_cases(self):
        self.assertEqual(max_value(5), 5)
        self.assertEqual(max_value(0, 0, 0), 0)


class TestMinValue(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(min_value(1, 2, 3), 1)
        self.assertEqual(min_value(-1, -2, -3), -3)

    def test_edge_cases(self):
        self.assertEqual(min_value(5), 5)
        self.assertEqual(min_value(0, 0, 0), 0)


class TestGetDecimalLength(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(get_decimal_length(1.234), 3)
        self.assertEqual(get_decimal_length(10), 0)
        self.assertEqual(get_decimal_length(0.1), 1)

    def test_edge_cases(self):
        self.assertEqual(get_decimal_length(0), 0)
        self.assertEqual(get_decimal_length(100), 0)

    def test_docstring_example(self):
        self.assertEqual(get_decimal_length(1.23), 2)
        self.assertEqual(get_decimal_length(100), 0)


class TestIsDouble(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_double(1.5))
        self.assertFalse(is_double(1))

    def test_with_loose_flag(self):
        self.assertTrue(is_double(0.1))
        self.assertTrue(is_double("0.1"))
        self.assertFalse(is_double("0.1", False))

    def test_docstring_example(self):
        self.assertTrue(is_double(0.1))
        self.assertTrue(is_double("0.1"))
        self.assertFalse(is_double("0.1", False))


class TestFactorize(unittest.TestCase):
    def test_basic_cases(self):
        result = factorize(12)
        self.assertEqual(result, [2, 2, 3])

    def test_edge_cases(self):
        self.assertEqual(factorize(1), [])
        self.assertEqual(factorize(7), [7])

    def test_docstring_example(self):
        self.assertEqual(factorize(12), [2, 2, 3])


class TestQuotient(unittest.TestCase):
    def test_basic_cases(self):
        result = quotient(10, 3)
        self.assertEqual(result[0], 3.0)
        self.assertEqual(result[1], 1)

    def test_edge_cases(self):
        result = quotient(9, 3)
        self.assertEqual(result[0], 3.0)
        self.assertEqual(result[1], 0)

    def test_docstring_example(self):
        self.assertEqual(quotient(5, 2), [2.0, 1.0])


class TestMultiples(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(multiples(2, 5), [2, 4, 6, 8, 10])

    def test_edge_cases(self):
        self.assertEqual(multiples(3, 3), [3, 6, 9])


class TestPercentile(unittest.TestCase):
    def test_basic_cases(self):
        data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        self.assertAlmostEqual(percentile(data, 50), 5.5)

    def test_edge_cases(self):
        data = [1, 2, 3, 4, 5]
        self.assertEqual(percentile(data, 0), 1)
        self.assertEqual(percentile(data, 100), 5)


class TestReduceFraction(unittest.TestCase):
    def test_basic_cases(self):
        result = reduce_fraction(4, 8)
        self.assertEqual(result["x"], 1.0)
        self.assertEqual(result["y"], 2.0)
        self.assertEqual(result["gcd"], 4.0)

    def test_edge_cases(self):
        result = reduce_fraction(2, 4)
        self.assertEqual(result["x"], 1)
        self.assertEqual(result["y"], 2)
        self.assertEqual(result["gcd"], 2)

    def test_docstring_example(self):
        result = reduce_fraction(2, 4)
        self.assertEqual(result, {"x": 1, "y": 2, "gcd": 2})


class TestBitwise(unittest.TestCase):
    def test_left_rotation(self):
        result = bitwise(0x12345678, 8)
        self.assertEqual(hex(result), "0x34567812")

    def test_right_rotation(self):
        result = bitwise(0x12345678, 8, "right")
        self.assertEqual(hex(result), "0x78123456")

    def test_docstring_example(self):
        self.assertEqual(hex(bitwise(0x12345678, 8)), "0x34567812")
        self.assertEqual(hex(bitwise(0x12345678, 8, "right")), "0x78123456")


class TestCorrelationCoefficient(unittest.TestCase):
    def test_basic_cases(self):
        x_values = [1, 2, 3, 4, 5]
        y_values = [2, 4, 6, 8, 10]
        self.assertAlmostEqual(correlation_coefficient(x_values, y_values), 1.0)

    def test_negative_correlation(self):
        x_values = [1, 2, 3, 4, 5]
        y_values = [10, 8, 6, 4, 2]
        self.assertAlmostEqual(correlation_coefficient(x_values, y_values), -1.0)


class TestDeviationValue(unittest.TestCase):
    def test_basic_cases(self):
        value = 70
        avg = 60
        std_dev = 10
        self.assertEqual(deviation_value(value, avg, std_dev), 60)


class TestToBaseN(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(to_base_n(10, 2), "1010")
        self.assertEqual(to_base_n(255, 16), "ff")

    def test_edge_cases(self):
        self.assertEqual(to_base_n(0, 2), "0")


class TestToCelsius(unittest.TestCase):
    def test_basic_cases(self):
        self.assertAlmostEqual(to_celsius(273.15), 0)
        self.assertAlmostEqual(to_celsius(373.15), 100)

    def test_docstring_example(self):
        self.assertAlmostEqual(to_celsius(300), 26.85)
        self.assertEqual(to_celsius(273.15), 0.0)
        self.assertEqual(to_celsius(0), -273.15)


class TestToKelvin(unittest.TestCase):
    def test_basic_cases(self):
        self.assertAlmostEqual(to_kelvin(0), 273.15)
        self.assertAlmostEqual(to_kelvin(100), 373.15)


if __name__ == "__main__":
    unittest.main()
