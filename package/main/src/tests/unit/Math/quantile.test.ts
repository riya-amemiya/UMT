import { percentile } from "@/Math/percentile";
import { quantile } from "@/Math/quantile";

describe("quantile function", () => {
  it("should return quartiles", () => {
    expect(quantile([1, 2, 3, 4, 5], 0.25)).toBe(2);
    expect(quantile([1, 2, 3, 4, 5], 0.5)).toBe(3);
    expect(quantile([1, 2, 3, 4, 5], 0.75)).toBe(4);
  });

  it("should return the minimum and maximum at 0 and 1", () => {
    expect(quantile([5, 1, 4, 2, 3], 0)).toBe(1);
    expect(quantile([5, 1, 4, 2, 3], 1)).toBe(5);
  });

  it("should interpolate between values", () => {
    expect(quantile([1, 2, 3, 4], 0.5)).toBe(2.5);
    expect(quantile([10, 20], 0.1)).toBe(11);
  });

  it("should match percentile with q * 100", () => {
    const values = Array.from({ length: 101 }, (_, index) => index * 1.5);
    expect(quantile(values, 0.29)).toBe(percentile(values, 29));
    expect(quantile(values, 0.07)).toBe(percentile(values, 7));
  });

  it("should return NaN for an empty array", () => {
    expect(quantile([], 0.5)).toBeNaN();
  });
});
