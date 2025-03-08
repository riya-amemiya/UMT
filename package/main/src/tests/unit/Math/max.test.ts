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
});
