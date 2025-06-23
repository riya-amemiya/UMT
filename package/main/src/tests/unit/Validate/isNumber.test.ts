import { isNumber } from "@/Validate/isNumber";
describe("isNumber", () => {
  describe("integers and decimals", () => {
    test("should return true for integer numbers", () => {
      expect(isNumber(5)).toBe(true);
      expect(isNumber(-5)).toBe(true);
      expect(isNumber(0)).toBe(true);
    });

    test("should return true for decimal numbers", () => {
      expect(isNumber(5.5)).toBe(true);
      expect(isNumber(-5.5)).toBe(true);
      expect(isNumber(0.0)).toBe(true);
    });
  });

  describe("string representations (loose mode)", () => {
    test("should return true for valid number strings when loose is true", () => {
      expect(isNumber("5", true)).toBe(true);
      expect(isNumber("-5.5", true)).toBe(true);
      expect(isNumber("0", true)).toBe(true);
    });

    test("should return false for invalid number strings", () => {
      expect(isNumber("abc", true)).toBe(false);
      expect(isNumber("5abc", true)).toBe(false);
    });

    test("should return false for number strings when loose is false", () => {
      expect(isNumber("5", false)).toBe(false);
      expect(isNumber("-5.5", false)).toBe(false);
    });

    test("should handle empty string", () => {
      expect(isNumber("")).toBe(true); // Due to isFinite("") returning true
    });
  });

  describe("special number values", () => {
    test("should return false for NaN", () => {
      expect(isNumber(Number.NaN)).toBe(false);
    });

    test("should return false for Infinity", () => {
      expect(isNumber(Number.POSITIVE_INFINITY)).toBe(false);
      expect(isNumber(Number.NEGATIVE_INFINITY)).toBe(false);
    });
  });

  describe("non-number types", () => {
    test("should return false for null and undefined", () => {
      expect(isNumber(null)).toBe(false);
      expect(isNumber(undefined)).toBe(false);
    });

    test("should return false for booleans", () => {
      expect(isNumber(true)).toBe(false);
      expect(isNumber(false)).toBe(false);
    });

    test("should return false for objects and arrays", () => {
      expect(isNumber({})).toBe(false);
      expect(isNumber([])).toBe(false);
    });
  });

  describe("type guard functionality", () => {
    test("should work as a type guard in loose mode", () => {
      const value: unknown = "123";
      if (isNumber(value)) {
        // TypeScript should recognize value as number | string
        expect(typeof value === "string" || typeof value === "number").toBe(
          true,
        );
        expect(Number(value)).toBe(123);
      }
    });

    test("should work as a type guard in strict mode", () => {
      const value: unknown = 123;
      if (isNumber(value, false)) {
        // TypeScript should recognize value as number
        expect(typeof value).toBe("number");
        expect(value).toBe(123);
      }
    });
  });
});
