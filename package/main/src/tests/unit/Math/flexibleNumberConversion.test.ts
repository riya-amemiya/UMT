import { flexibleNumberConversion } from "@/Math/flexibleNumberConversion";

describe("flexibleNumberConversion", () => {
  // Basic numeric input
  describe("Basic numeric input", () => {
    test("should return positive integers as-is", () => {
      expect(flexibleNumberConversion(123)).toBe(123);
      expect(flexibleNumberConversion(456)).toBe(456);
      expect(flexibleNumberConversion(789)).toBe(789);
    });

    test("should return negative integers as-is", () => {
      expect(flexibleNumberConversion(-123)).toBe(-123);
      expect(flexibleNumberConversion(-456)).toBe(-456);
      expect(flexibleNumberConversion(-789)).toBe(-789);
    });

    test("should return zero as-is", () => {
      expect(flexibleNumberConversion(0)).toBe(0);
      expect(flexibleNumberConversion(-0)).toBe(-0);
    });
  });

  // String numeric input
  describe("String numeric input", () => {
    test("should properly convert positive integer strings", () => {
      expect(flexibleNumberConversion("123")).toBe(123);
      expect(flexibleNumberConversion("456")).toBe(456);
      expect(flexibleNumberConversion("789")).toBe(789);
    });

    test("should properly convert negative integer strings", () => {
      expect(flexibleNumberConversion("-123")).toBe(-123);
      expect(flexibleNumberConversion("-456")).toBe(-456);
      expect(flexibleNumberConversion("-789")).toBe(-789);
    });

    test("should properly convert decimal strings", () => {
      expect(flexibleNumberConversion("3.14")).toBe(3.14);
      expect(flexibleNumberConversion("-2.718")).toBe(-2.718);
      expect(flexibleNumberConversion("0.001")).toBe(0.001);
    });

    test("should correctly interpret exponential notation strings", () => {
      expect(flexibleNumberConversion("1e3")).toBe(1000);
      expect(flexibleNumberConversion("-2.5e-3")).toBe(-0.0025);
      expect(flexibleNumberConversion("6.022e23")).toBe(6.022e23);
    });

    test("should correctly interpret special base notation strings", () => {
      expect(flexibleNumberConversion("0x1A")).toBe(26);
      expect(flexibleNumberConversion("0b1011")).toBe(11);
      expect(flexibleNumberConversion("0o77")).toBe(63);
    });

    test("should properly convert strings with spaces around numbers", () => {
      expect(flexibleNumberConversion(" 123 ")).toBe(123);
      expect(flexibleNumberConversion("   -456")).toBe(-456);
      expect(flexibleNumberConversion("789   ")).toBe(789);
    });

    test("should handle strings with only plus sign", () => {
      expect(flexibleNumberConversion("+")).toBe(Number.NaN);
      expect(flexibleNumberConversion("+123")).toBe(123);
      expect(flexibleNumberConversion("+0")).toBe(0);
    });

    test("should handle strings with invalid exponential notation", () => {
      expect(flexibleNumberConversion("1e")).toBe(1);
      expect(flexibleNumberConversion("e10")).toBe(Number.NaN);
      expect(flexibleNumberConversion("2e+")).toBe(2);
    });

    test("should handle strings with only decimal point", () => {
      expect(flexibleNumberConversion(".")).toBe(Number.NaN);
      expect(flexibleNumberConversion("-.")).toBe(Number.NaN);
      expect(flexibleNumberConversion(".e1")).toBe(Number.NaN);
    });

    test("should handle strings that don't start with a number", () => {
      expect(flexibleNumberConversion("px42")).toBe(Number.NaN);
      expect(flexibleNumberConversion("meter3.14")).toBe(Number.NaN);
      expect(flexibleNumberConversion("abc123")).toBe(Number.NaN);
    });
  });

  // Special numeric input
  describe("Special numeric input", () => {
    test("should properly handle Infinity", () => {
      expect(flexibleNumberConversion("Infinity")).toBe(
        Number.POSITIVE_INFINITY,
      );
      expect(flexibleNumberConversion("-Infinity")).toBe(
        Number.NEGATIVE_INFINITY,
      );
      expect(flexibleNumberConversion(Number.POSITIVE_INFINITY)).toBe(
        Number.POSITIVE_INFINITY,
      );
      expect(flexibleNumberConversion(Number.NEGATIVE_INFINITY)).toBe(
        Number.NEGATIVE_INFINITY,
      );
    });

    test("should handle NaN input", () => {
      expect(flexibleNumberConversion(Number.NaN)).toBe(Number.NaN);
      expect(flexibleNumberConversion("NaN")).toBe(Number.NaN);
    });

    test("should properly handle very large numbers", () => {
      expect(flexibleNumberConversion(Number.MAX_SAFE_INTEGER)).toBe(
        Number.MAX_SAFE_INTEGER,
      );
      expect(flexibleNumberConversion("9007199254740991")).toBe(
        9007199254740991,
      );
      expect(flexibleNumberConversion("1e308")).toBe(1e308);
    });

    test("should properly handle very small numbers", () => {
      expect(flexibleNumberConversion(Number.MIN_VALUE)).toBe(Number.MIN_VALUE);
      expect(flexibleNumberConversion("5e-324")).toBe(5e-324);
      expect(flexibleNumberConversion("-1e-308")).toBe(-1e-308);
    });
  });

  // Invalid or special input
  describe("Invalid or special input", () => {
    test("should treat empty string, null, undefined as 0", () => {
      expect(flexibleNumberConversion("")).toBe(0);
      expect(flexibleNumberConversion(null)).toBe(0);
      expect(flexibleNumberConversion(undefined)).toBe(0);
    });

    test("should return NaN for invalid strings", () => {
      expect(flexibleNumberConversion("not a number")).toBe(Number.NaN);
      expect(flexibleNumberConversion("abc")).toBe(Number.NaN);
      expect(flexibleNumberConversion("abc123")).toBe(Number.NaN);
    });

    test("should return NaN for objects and arrays", () => {
      expect(flexibleNumberConversion({})).toBe(Number.NaN);
      expect(flexibleNumberConversion({ key: "value" })).toBe(Number.NaN);
      expect(flexibleNumberConversion([])).toBe(Number.NaN);
      expect(flexibleNumberConversion([1, 2, 3])).toBe(Number.NaN);
    });

    test("should return NaN for functions", () => {
      const func = () => {};
      expect(flexibleNumberConversion(func)).toBe(Number.NaN);
      expect(flexibleNumberConversion(() => {})).toBe(Number.NaN);
      expect(flexibleNumberConversion(() => {})).toBe(Number.NaN);
    });

    test("should return NaN for strings with special characters", () => {
      expect(flexibleNumberConversion("@123")).toBe(Number.NaN);
      expect(flexibleNumberConversion("!@#$%")).toBe(Number.NaN);
    });
  });

  // Complex cases
  describe("Complex cases", () => {
    test("should extract leading number from strings with mixed numbers and characters", () => {
      expect(flexibleNumberConversion("42px")).toBe(42);
      expect(flexibleNumberConversion("-42px")).toBe(-42);
      expect(flexibleNumberConversion("3.14meters")).toBe(3.14);
      expect(flexibleNumberConversion("-3.14meters")).toBe(-3.14);
      expect(flexibleNumberConversion("1e10meters")).toBe(1e10);
      expect(flexibleNumberConversion("-1e10meters")).toBe(-1e10);
    });

    test("should properly handle strings with internal spaces", () => {
      expect(flexibleNumberConversion(" 123 456 ")).toBe(123);
      expect(flexibleNumberConversion(" -456px ")).toBe(-456);
      expect(flexibleNumberConversion("3.14 meters")).toBe(3.14);
      expect(flexibleNumberConversion("1e2 meters")).toBe(100);
    });

    test("should properly handle numbers with leading zeros", () => {
      expect(flexibleNumberConversion("0123")).toBe(123);
      expect(flexibleNumberConversion("-0456")).toBe(-456);
      expect(flexibleNumberConversion("000789")).toBe(789);
    });

    test("should properly handle numbers with leading plus sign", () => {
      expect(flexibleNumberConversion("+123")).toBe(123);
      expect(flexibleNumberConversion("+0")).toBe(0);
      expect(flexibleNumberConversion("+3.14")).toBe(3.14);
      expect(flexibleNumberConversion("+1e10")).toBe(1e10);
    });
  });

  // Performance and stress tests
  describe("Performance and stress tests", () => {
    test("should handle large sequences of valid inputs", () => {
      for (let i = 0; i < 1000; i++) {
        expect(flexibleNumberConversion(i.toString())).toBe(i);
      }
    });

    test("should handle large sequences of invalid inputs", () => {
      for (let i = 0; i < 1000; i++) {
        expect(flexibleNumberConversion(`invalid${i}`)).toBe(Number.NaN);
      }
    });
  });

  // Boundary value tests
  describe("Boundary value tests", () => {
    test("should handle Number.MAX_VALUE", () => {
      expect(flexibleNumberConversion(Number.MAX_VALUE)).toBe(Number.MAX_VALUE);
      expect(flexibleNumberConversion("1.7976931348623157e+308")).toBe(
        Number.MAX_VALUE,
      );
    });

    test("should handle Number.MIN_VALUE", () => {
      expect(flexibleNumberConversion(Number.MIN_VALUE)).toBe(Number.MIN_VALUE);
      expect(flexibleNumberConversion("5e-324")).toBe(Number.MIN_VALUE);
    });

    test("should handle Number.MAX_SAFE_INTEGER", () => {
      expect(flexibleNumberConversion(Number.MAX_SAFE_INTEGER)).toBe(
        Number.MAX_SAFE_INTEGER,
      );
      expect(flexibleNumberConversion("9007199254740991")).toBe(
        Number.MAX_SAFE_INTEGER,
      );
    });

    test("should handle Number.MIN_SAFE_INTEGER", () => {
      expect(flexibleNumberConversion(Number.MIN_SAFE_INTEGER)).toBe(
        Number.MIN_SAFE_INTEGER,
      );
      expect(flexibleNumberConversion("-9007199254740991")).toBe(
        Number.MIN_SAFE_INTEGER,
      );
    });
  });
});
