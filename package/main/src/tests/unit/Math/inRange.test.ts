import { inRange } from "@/Math/inRange";

describe("inRange", () => {
  it("should return true when value is within [0, end)", () => {
    expect(inRange(3, 5)).toBe(true);
  });

  it("should return false when value equals end (two-arg form)", () => {
    expect(inRange(5, 5)).toBe(false);
  });

  it("should return true when value equals 0 (two-arg form)", () => {
    expect(inRange(0, 5)).toBe(true);
  });

  it("should return false for negative value (two-arg form)", () => {
    expect(inRange(-1, 5)).toBe(false);
  });

  it("should return true when value is within [start, end)", () => {
    expect(inRange(3, 2, 5)).toBe(true);
  });

  it("should swap start and end if start > end", () => {
    expect(inRange(3, 5, 2)).toBe(true);
  });

  it("should return false when value equals the upper bound", () => {
    expect(inRange(5, 2, 5)).toBe(false);
  });

  it("should return true when value equals the lower bound", () => {
    expect(inRange(2, 2, 5)).toBe(true);
  });

  it("should handle negative ranges", () => {
    expect(inRange(-3, -5, -1)).toBe(true);
    expect(inRange(-5, -5, -1)).toBe(true);
    expect(inRange(-1, -5, -1)).toBe(false);
  });

  it("should handle floating point values", () => {
    expect(inRange(0.5, 0, 1)).toBe(true);
    expect(inRange(1.5, 0, 1)).toBe(false);
  });

  it("should handle negative ranges (two-arg form)", () => {
    expect(inRange(-3, -5)).toBe(true);
    expect(inRange(0, -5)).toBe(false);
    expect(inRange(-5, -5)).toBe(true);
  });
});
