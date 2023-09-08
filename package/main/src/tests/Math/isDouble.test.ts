import { isDouble } from "@/Math/isDouble";
describe("isDouble", () => {
  it("should return true for valid doubles", () => {
    expect(isDouble(1.5)).toBe(true);
    expect(isDouble("1.5")).toBe(true);
    expect(isDouble("1.5", false)).toBe(false);
  });

  it("should return false for invalid doubles", () => {
    expect(isDouble(1)).toBe(false);
    expect(isDouble("1")).toBe(false);
    expect(isDouble("1", false)).toBe(false);
    expect(isDouble(NaN)).toBe(false);
    expect(isDouble(Infinity)).toBe(false);
    expect(isDouble(-Infinity)).toBe(false);
    expect(isDouble(null)).toBe(false);
    expect(isDouble(undefined)).toBe(false);
    expect(isDouble(true)).toBe(false);
    expect(isDouble(false)).toBe(false);
    expect(isDouble({})).toBe(false);
    expect(isDouble([])).toBe(false);
    expect(isDouble(0x12)).toBe(false);
  });
});
