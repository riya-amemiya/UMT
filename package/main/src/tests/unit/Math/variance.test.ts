import { variance } from "@/Math/variance";

describe("variance function", () => {
  it("should calculate the population variance by default", () => {
    expect(variance([1, 2, 3])).toBe(0.666_666_666_666_666_6);
    expect(variance([1, 2, 3, 4, 5])).toBe(2);
    expect(variance([10, 12, 23, 23, 16, 23, 21, 16])).toBe(24);
  });

  it("should calculate the population variance when sample is false", () => {
    expect(variance([1, 2, 3, 4, 5], false)).toBe(2);
  });

  it("should calculate the sample variance when sample is true", () => {
    expect(variance([1, 2, 3], true)).toBe(1);
    expect(variance([1, 2, 3, 4, 5], true)).toBe(2.5);
    expect(variance([10, 12, 23, 23, 16, 23, 21, 16], true)).toBe(
      27.428_571_428_571_427,
    );
  });

  it("should avoid floating point errors for decimal values", () => {
    expect(variance([0.1, 0.2, 0.3])).toBe(0.006_666_666_666_666_666);
  });

  it("should return 0 for identical values", () => {
    expect(variance([5, 5, 5, 5])).toBe(0);
    expect(variance([5, 5, 5, 5], true)).toBe(0);
  });

  it("should return 0 for a single value population", () => {
    expect(variance([42])).toBe(0);
  });

  it("should return NaN for a single value sample", () => {
    expect(variance([42], true)).toBeNaN();
  });

  it("should return 0 for an empty array", () => {
    expect(variance([])).toBe(0);
    expect(variance([], true)).toBe(0);
  });
});
