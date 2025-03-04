import { isValueNaN } from "@/Validate/isValueNaN";
describe("isValueNaN function", () => {
  describe("strict mode (loose = false)", () => {
    it("should return false for valid numbers", () => {
      expect(isValueNaN(0)).toBe(false);
      expect(isValueNaN(1)).toBe(false);
      expect(isValueNaN(-1)).toBe(false);
      expect(isValueNaN(Number.POSITIVE_INFINITY)).toBe(false);
    });

    it("should return true for NaN", () => {
      expect(isValueNaN(Number.NaN)).toBe(true);
      expect(isValueNaN(Number.parseInt("not a number"))).toBe(true);
    });

    it("should return false for string values", () => {
      expect(isValueNaN("NaN")).toBe(false);
      expect(isValueNaN("0")).toBe(false);
      expect(isValueNaN("a")).toBe(false);
      expect(isValueNaN("")).toBe(false);
    });

    it("should return false for non-number types", () => {
      expect(isValueNaN(undefined)).toBe(false);
      expect(isValueNaN(null)).toBe(false);
      expect(isValueNaN({})).toBe(false);
      expect(isValueNaN([])).toBe(false);
    });
  });

  describe("loose mode (loose = true)", () => {
    it("should return false for valid number strings", () => {
      expect(isValueNaN("0", true)).toBe(false);
      expect(isValueNaN("1", true)).toBe(false);
      expect(isValueNaN("-1", true)).toBe(false);
      expect(isValueNaN("Infinity", true)).toBe(false);
    });

    it("should return true for NaN and NaN strings", () => {
      expect(isValueNaN(Number.NaN, true)).toBe(true);
      expect(isValueNaN("NaN", true)).toBe(true);
      expect(isValueNaN(Number.parseInt("not a number"), true)).toBe(true);
    });

    it("should handle non-numeric strings correctly", () => {
      expect(isValueNaN("a", true)).toBe(true);
      expect(isValueNaN("abc", true)).toBe(true);
      expect(isValueNaN("", true)).toBe(false); // isNaN("") returns false
      expect(isValueNaN(" ", true)).toBe(false); // isNaN(" ") returns false (coerced to 0)
    });

    it("should handle non-string/non-number types", () => {
      expect(isValueNaN(undefined, true)).toBe(true);
      expect(isValueNaN(null, true)).toBe(false);
      expect(isValueNaN({}, true)).toBe(true);
      expect(isValueNaN([], true)).toBe(false);
    });
  });
});
