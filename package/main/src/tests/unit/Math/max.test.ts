import { max } from "@/Math/max";
describe("max function", () => {
  it("should return the maximum value", () => {
    expect(max(1, 2, 3)).toBe(3);
    expect(max(3, 2, 1)).toBe(3);
    expect(max(-1, -2, -3)).toBe(-1);
  });

  it("should remove duplicates before finding maximum", () => {
    expect(max(1, 1, 1)).toBe(1);
    expect(max(1, 2, 2, 3, 3)).toBe(3);
  });

  it("should handle single value", () => {
    expect(max(5)).toBe(5);
    expect(max(-5)).toBe(-5);
  });

  it("should handle decimal numbers", () => {
    expect(max(1.5, 2.5, 1.1)).toBe(2.5);
    expect(max(-1.5, -2.5, -1.1)).toBe(-1.1);
  });

  describe("special number values", () => {
    it("should return NaN if any argument is NaN", () => {
      expect(max(1, Number.NaN, 3)).toBe(Number.NaN);
      expect(max(Number.NaN)).toBe(Number.NaN);
    });

    it("should handle Infinity", () => {
      expect(max(1, Number.POSITIVE_INFINITY, 3)).toBe(
        Number.POSITIVE_INFINITY,
      );
      expect(max(Number.NEGATIVE_INFINITY, 0, 1)).toBe(1);
    });

    it("should handle -Infinity", () => {
      expect(max(Number.NEGATIVE_INFINITY, Number.NEGATIVE_INFINITY)).toBe(
        Number.NEGATIVE_INFINITY,
      );
    });

    it("should return -Infinity for empty arguments", () => {
      expect(max()).toBe(Number.NEGATIVE_INFINITY);
    });
  });
});
