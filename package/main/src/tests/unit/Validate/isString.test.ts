import { isString } from "@/Validate/isString";

describe("isString", () => {
  describe("string values", () => {
    test("should return true for empty string", () => {
      expect(isString("")).toBe(true);
    });

    test("should return true for regular strings", () => {
      expect(isString("test")).toBe(true);
      expect(isString("hello world")).toBe(true);
      expect(isString("123")).toBe(true);
    });

    test("should return true for String constructor result", () => {
      expect(isString(String("test"))).toBe(true);
      expect(isString(String(123))).toBe(true);
    });

    test("should return true for template literals", () => {
      const value = "world";
      expect(isString(`hello ${value}`)).toBe(true);
    });
  });

  describe("non-string values", () => {
    test("should return false for numbers", () => {
      expect(isString(123)).toBe(false);
      expect(isString(0)).toBe(false);
      expect(isString(-1)).toBe(false);
      expect(isString(3.14)).toBe(false);
    });

    test("should return false for null and undefined", () => {
      expect(isString(null)).toBe(false);
      expect(isString(undefined)).toBe(false);
    });

    test("should return false for booleans", () => {
      expect(isString(true)).toBe(false);
      expect(isString(false)).toBe(false);
    });

    test("should return false for objects and arrays", () => {
      expect(isString({})).toBe(false);
      expect(isString([])).toBe(false);
      expect(isString({ toString: () => "test" })).toBe(false);
    });

    test("should return false for functions", () => {
      expect(isString(() => "test")).toBe(false);
    });

    test("should return false for symbols", () => {
      expect(isString(Symbol("test"))).toBe(false);
    });
  });

  describe("type guard functionality", () => {
    test("should work as a type guard", () => {
      const value: unknown = "test";
      if (isString(value)) {
        expect(value.toUpperCase()).toBe("TEST");
      }
    });
  });
});
