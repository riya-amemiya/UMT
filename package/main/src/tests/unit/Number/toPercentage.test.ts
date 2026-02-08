import { toPercentage } from "@/Number/toPercentage";

describe("toPercentage", () => {
  it("should calculate basic percentage", () => {
    expect(toPercentage(25, 100)).toBe(25);
    expect(toPercentage(50, 100)).toBe(50);
    expect(toPercentage(100, 100)).toBe(100);
  });

  it("should return result with 2 decimal places by default", () => {
    expect(toPercentage(1, 3)).toBe(33.33);
    expect(toPercentage(2, 3)).toBe(66.67);
  });

  it("should respect custom decimal places", () => {
    expect(toPercentage(1, 3, 0)).toBe(33);
    expect(toPercentage(1, 3, 1)).toBe(33.3);
    expect(toPercentage(1, 3, 4)).toBe(33.3333);
  });

  it("should return 0 when total is 0", () => {
    expect(toPercentage(0, 0)).toBe(0);
    expect(toPercentage(5, 0)).toBe(0);
    expect(toPercentage(-5, 0)).toBe(0);
  });

  it("should handle 0 value", () => {
    expect(toPercentage(0, 100)).toBe(0);
  });

  it("should handle values greater than total", () => {
    expect(toPercentage(150, 100)).toBe(150);
    expect(toPercentage(200, 100)).toBe(200);
  });

  it("should handle negative values", () => {
    expect(toPercentage(-25, 100)).toBe(-25);
  });
});
