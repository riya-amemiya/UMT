import { correlationCoefficient } from "@/Math/correlationCoefficient";

describe("correlationCoefficient", () => {
  it("should calculate correlation according to JSDoc examples", () => {
    expect(correlationCoefficient([1, 2, 3, 4, 5], [2, 4, 6, 8, 10])).toBe(1);
    expect(correlationCoefficient([1, 2, 3, 4, 5], [5, 4, 3, 2, 1])).toBe(-1);
    expect(correlationCoefficient([1, 2, 3, 4, 5], [1, 1, 1, 1, 1])).toBeNaN();
  });

  it("should return NaN for empty arrays", () => {
    expect(correlationCoefficient([], [])).toBeNaN();
  });

  it("should return NaN for single element arrays", () => {
    expect(correlationCoefficient([1], [2])).toBeNaN();
  });

  it("should handle partial correlation", () => {
    const result = correlationCoefficient([1, 2, 3, 4, 5], [2, 3, 5, 4, 6]);
    expect(result).toBeGreaterThan(0);
    expect(result).toBeLessThan(1);
  });

  it("should handle negative numbers", () => {
    expect(correlationCoefficient([-1, -2, -3], [-2, -4, -6])).toBe(1);
  });

  it("should handle decimal numbers", () => {
    expect(correlationCoefficient([1.5, 2.5, 3.5], [3.0, 5.0, 7.0])).toBe(1);
  });

  it("should return NaN when arrays have no variance", () => {
    expect(correlationCoefficient([1, 1, 1], [1, 2, 3])).toBeNaN();
    expect(correlationCoefficient([1, 2, 3], [2, 2, 2])).toBeNaN();
  });
});
