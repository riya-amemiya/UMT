import { factorial } from "@/Math/factorial";

describe("factorial function", () => {
  it("should return 1 for 0", () => {
    expect(factorial(0)).toBe(1);
  });

  it("should return 1 for 1", () => {
    expect(factorial(1)).toBe(1);
  });

  it("should return 120 for 5", () => {
    expect(factorial(5)).toBe(120);
  });

  it("should return 3628800 for 10", () => {
    expect(factorial(10)).toBe(3628800);
  });
});
