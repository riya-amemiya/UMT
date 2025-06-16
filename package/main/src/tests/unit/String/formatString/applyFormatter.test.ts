import { applyFormatter } from "@/String/formatString/applyFormatter";

describe("applyFormatter function", () => {
  const mockFormatters = {
    upper: (value: unknown) => String(value).toUpperCase(),
    lower: (value: unknown) => String(value).toLowerCase(),
    pad: (value: unknown, length: string, char: string) =>
      String(value).padStart(Number(length), char),
    currency: (value: unknown, locale: string, currency: string) =>
      new Intl.NumberFormat(locale, {
        style: "currency",
        currency: currency,
      }).format(Number(value)),
    multiply: (value: unknown, factor: string) =>
      String(Number(value) * Number(factor)),
  };

  describe("simple formatters without arguments", () => {
    test("should apply uppercase formatter", () => {
      expect(applyFormatter("hello", "upper", mockFormatters)).toBe("HELLO");
      expect(applyFormatter("WORLD", "upper", mockFormatters)).toBe("WORLD");
      expect(applyFormatter(123, "upper", mockFormatters)).toBe("123");
    });

    test("should apply lowercase formatter", () => {
      expect(applyFormatter("HELLO", "lower", mockFormatters)).toBe("hello");
      expect(applyFormatter("WoRlD", "lower", mockFormatters)).toBe("world");
      expect(applyFormatter(456, "lower", mockFormatters)).toBe("456");
    });
  });

  describe("formatters with arguments", () => {
    test("should parse and apply formatter with single argument", () => {
      expect(applyFormatter(42, "multiply(2)", mockFormatters)).toBe("84");
      expect(applyFormatter(10, "multiply(0.5)", mockFormatters)).toBe("5");
    });

    test("should parse and apply formatter with multiple arguments", () => {
      expect(applyFormatter(42, "pad(4,0)", mockFormatters)).toBe("0042");
      expect(applyFormatter(7, "pad(3, )", mockFormatters)).toBe("  7");
      expect(applyFormatter("test", "pad(6,*)", mockFormatters)).toBe("**test");
    });

    test("should handle quoted arguments", () => {
      expect(applyFormatter(5, 'pad(3,"x")', mockFormatters)).toBe("xx5");
      expect(applyFormatter(5, "pad(3,'y')", mockFormatters)).toBe("yy5");
    });

    test("should handle currency formatter with locale and currency", () => {
      const result = applyFormatter(
        1234.56,
        "currency(en-US,USD)",
        mockFormatters,
      );
      expect(result).toMatch(/\$1,234\.56/);
    });

    test("should trim whitespace in arguments", () => {
      expect(applyFormatter(42, "pad( 4 , 0 )", mockFormatters)).toBe("0042");
      expect(applyFormatter(5, "multiply( 3 )", mockFormatters)).toBe("15");
    });
  });

  describe("invalid formatter strings", () => {
    test("should return string value for invalid formatter syntax", () => {
      expect(applyFormatter("test", "invalid!@#", mockFormatters)).toBe("test");
      expect(applyFormatter(123, "bad-format", mockFormatters)).toBe("123");
      expect(applyFormatter("hello", "upper(", mockFormatters)).toBe("hello");
      expect(applyFormatter("world", "lower)", mockFormatters)).toBe("world");
    });

    test("should return string value for non-existent formatter", () => {
      expect(applyFormatter("test", "nonexistent", mockFormatters)).toBe(
        "test",
      );
      expect(applyFormatter(456, "missing(arg)", mockFormatters)).toBe("456");
    });
  });

  describe("edge cases", () => {
    test("should handle empty arguments", () => {
      expect(applyFormatter("test", "upper()", mockFormatters)).toBe("TEST");
    });

    test("should handle different value types", () => {
      expect(applyFormatter(null, "upper", mockFormatters)).toBe("NULL");
      expect(applyFormatter(undefined, "upper", mockFormatters)).toBe(
        "UNDEFINED",
      );
      expect(applyFormatter(true, "upper", mockFormatters)).toBe("TRUE");
      expect(applyFormatter(false, "lower", mockFormatters)).toBe("false");
      expect(applyFormatter({}, "upper", mockFormatters)).toBe(
        "[OBJECT OBJECT]",
      );
      expect(applyFormatter([1, 2, 3], "upper", mockFormatters)).toBe("1,2,3");
    });

    test("should handle empty formatters object", () => {
      expect(applyFormatter("test", "upper", {})).toBe("test");
      expect(applyFormatter(123, "nonexistent", {})).toBe("123");
    });

    test("should handle complex argument parsing", () => {
      expect(applyFormatter("x", "pad(5,abc)", mockFormatters)).toBe("abcax");
      expect(applyFormatter("y", 'pad(4,"z,w")', mockFormatters)).toBe("z,wy");
    });

    test("should handle empty arguments in argument list", () => {
      expect(applyFormatter("x", "pad(3,)", mockFormatters)).toBe("  x");
      expect(applyFormatter("y", "pad(4,,)", mockFormatters)).toBe("   y");
    });
  });

  describe("formatter name validation", () => {
    test("should only accept word characters in formatter names", () => {
      const invalidFormatters = {
        "with-dash": (v: unknown) => String(v),
        "with.dot": (v: unknown) => String(v),
        "with space": (v: unknown) => String(v),
      };

      expect(applyFormatter("test", "with-dash", invalidFormatters)).toBe(
        "test",
      );
      expect(applyFormatter("test", "with.dot", invalidFormatters)).toBe(
        "test",
      );
      expect(applyFormatter("test", "with space", invalidFormatters)).toBe(
        "test",
      );
    });
  });
});
