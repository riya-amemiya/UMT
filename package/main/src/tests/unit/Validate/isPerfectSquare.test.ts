import { isPerfectSquare } from "@/Validate/isPerfectSquare";

describe("isPerfectSquare", () => {
  it("should return true for a perfect square", () => {
    expect(isPerfectSquare(16)).toBe(true);
    expect(isPerfectSquare(25)).toBe(true);
    expect(isPerfectSquare(49)).toBe(true);
    expect(isPerfectSquare(0)).toBe(true);
  });

  it("should return false for a non-perfect square", () => {
    expect(isPerfectSquare(20)).toBe(false);
    expect(isPerfectSquare(-16)).toBe(false);
  });

  it("should return true for 1", () => {
    expect(isPerfectSquare(1)).toBe(true);
  });

  it("should handle large numbers", () => {
    expect(isPerfectSquare(100000000)).toBe(true);
    expect(isPerfectSquare(100000002)).toBe(false);
  });
});
