import { isDouble } from "@/Validate/isDouble";
describe("isDouble", () => {
  it("should return true for valid doubles", () => {
    expect(isDouble(1.5)).toBe(true);
    expect(isDouble(-1.5)).toBe(true);
    expect(isDouble(1.23e-4)).toBe(true);
    expect(isDouble("1.5")).toBe(true);
    expect(isDouble("1.5", false)).toBe(false);
  });

  it("should work as a type guard", () => {
    const value: unknown = 1.5;
    if (isDouble(value)) {
      // TypeScript should recognize value as number | string
      expect(typeof value === "number" || typeof value === "string").toBe(true);
      expect(Number(value)).toBe(1.5);
    }

    const strictValue: unknown = 1.5;
    if (isDouble(strictValue, false)) {
      // TypeScript should recognize strictValue as number
      expect(typeof strictValue).toBe("number");
      expect(strictValue).toBe(1.5);
    }
  });

  it("should return false for invalid doubles", () => {
    expect(isDouble(1)).toBe(false);
    expect(isDouble(1, false)).toBe(false);
    expect(isDouble("1")).toBe(false);
    expect(isDouble("1", false)).toBe(false);
    expect(isDouble(Number.NaN)).toBe(false);
    expect(isDouble(Number.POSITIVE_INFINITY)).toBe(false);
    expect(isDouble(Number.NEGATIVE_INFINITY)).toBe(false);
    expect(isDouble(null)).toBe(false);
    expect(isDouble(undefined)).toBe(false);
    expect(isDouble(true)).toBe(false);
    expect(isDouble(false)).toBe(false);
    expect(isDouble({})).toBe(false);
    expect(isDouble([])).toBe(false);
    expect(isDouble(0x12)).toBe(false);
    expect(isDouble(Number.NEGATIVE_INFINITY, false)).toBe(false);
    expect(isDouble(null, false)).toBe(false);
    expect(isDouble(true, false)).toBe(false);
    expect(isDouble(false, false)).toBe(false);
  });
});
