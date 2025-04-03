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
});
