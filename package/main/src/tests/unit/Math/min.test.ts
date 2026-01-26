import { min } from "@/Math/min";
describe("min function", () => {
  it("should return the minimum value", () => {
    expect(min(1, 2, 3)).toBe(1);
    expect(min(3, 2, 1)).toBe(1);
    expect(min(-1, -2, -3)).toBe(-3);
  });

  it("should remove duplicates before finding minimum", () => {
    expect(min(1, 1, 1)).toBe(1);
    expect(min(1, 2, 2, 3, 3)).toBe(1);
  });

  it("should handle single value", () => {
    expect(min(5)).toBe(5);
    expect(min(-5)).toBe(-5);
  });

  it("should handle decimal numbers", () => {
    expect(min(1.5, 2.5, 1.1)).toBe(1.1);
    expect(min(-1.5, -2.5, -1.1)).toBe(-2.5);
  });

  describe("special number values", () => {
    it("should return NaN if any argument is NaN", () => {
      expect(min(1, Number.NaN, 3)).toBe(Number.NaN);
      expect(min(Number.NaN)).toBe(Number.NaN);
    });

    it("should handle Infinity", () => {
      expect(min(1, Number.POSITIVE_INFINITY, 3)).toBe(1);
      expect(min(Number.POSITIVE_INFINITY, Number.POSITIVE_INFINITY)).toBe(
        Number.POSITIVE_INFINITY,
      );
    });

    it("should handle -Infinity", () => {
      expect(min(Number.NEGATIVE_INFINITY, 0, 1)).toBe(
        Number.NEGATIVE_INFINITY,
      );
    });

    it("should return Infinity for empty arguments", () => {
      expect(min()).toBe(Number.POSITIVE_INFINITY);
    });
  });
});
