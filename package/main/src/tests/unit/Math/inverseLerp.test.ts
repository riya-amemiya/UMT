import { inverseLerp } from "@/Math/inverseLerp";

describe("inverseLerp function", () => {
  it("should return 0 at start and 1 at end", () => {
    expect(inverseLerp(10, 20, 10)).toBe(0);
    expect(inverseLerp(10, 20, 20)).toBe(1);
  });

  it("should locate a value inside the range", () => {
    expect(inverseLerp(0, 10, 5)).toBe(0.5);
    expect(inverseLerp(10, 20, 12.5)).toBe(0.25);
  });

  it("should handle a descending range", () => {
    expect(inverseLerp(20, 10, 17.5)).toBe(0.25);
  });

  it("should return factors outside 0..1 for values outside the range", () => {
    expect(inverseLerp(0, 10, 20)).toBe(2);
    expect(inverseLerp(0, 10, -5)).toBe(-0.5);
  });

  it("should return 0 when start equals end", () => {
    expect(inverseLerp(5, 5, 7)).toBe(0);
    expect(inverseLerp(5, 5, 5)).toBe(0);
  });

  it("should propagate NaN", () => {
    expect(inverseLerp(0, 10, Number.NaN)).toBeNaN();
  });
});
