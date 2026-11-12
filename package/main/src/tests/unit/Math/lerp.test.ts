import { lerp } from "@/Math/lerp";

describe("lerp function", () => {
  it("should return start when t is 0", () => {
    expect(lerp(10, 20, 0)).toBe(10);
  });

  it("should return end when t is 1", () => {
    expect(lerp(10, 20, 1)).toBe(20);
  });

  it("should interpolate between start and end", () => {
    expect(lerp(0, 10, 0.5)).toBe(5);
    expect(lerp(10, 20, 0.25)).toBe(12.5);
    expect(lerp(-10, 10, 0.75)).toBe(5);
  });

  it("should interpolate a descending range", () => {
    expect(lerp(20, 10, 0.25)).toBe(17.5);
  });

  it("should extrapolate outside 0..1", () => {
    expect(lerp(0, 10, 2)).toBe(20);
    expect(lerp(0, 10, -0.5)).toBe(-5);
  });

  it("should return start for an empty range", () => {
    expect(lerp(5, 5, 0.3)).toBe(5);
  });

  it("should propagate NaN", () => {
    expect(lerp(0, 10, Number.NaN)).toBeNaN();
  });
});
