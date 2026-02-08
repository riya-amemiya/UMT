import { some } from "@/Predicate/some";

describe("some", () => {
  it("returns true when at least one predicate passes", () => {
    const isZeroOrNegative = some(
      (n: number) => n === 0,
      (n: number) => n < 0,
    );
    expect(isZeroOrNegative(0)).toBe(true);
    expect(isZeroOrNegative(-5)).toBe(true);
  });

  it("returns false when no predicate passes", () => {
    const isZeroOrNegative = some(
      (n: number) => n === 0,
      (n: number) => n < 0,
    );
    expect(isZeroOrNegative(5)).toBe(false);
  });

  it("short-circuits on first success", () => {
    let secondCalled = false;
    const combined = some(
      () => true,
      () => {
        secondCalled = true;
        return false;
      },
    );
    combined();
    expect(secondCalled).toBe(false);
  });

  it("handles single predicate", () => {
    const single = some((n: number) => n > 0);
    expect(single(1)).toBe(true);
    expect(single(-1)).toBe(false);
  });

  it("returns false for no predicates", () => {
    const never = some();
    expect(never()).toBe(false);
  });

  it("handles multiple arguments", () => {
    const eitherPositive = some(
      (a: number, _b: number) => a > 0,
      (_a: number, b: number) => b > 0,
    );
    expect(eitherPositive(1, -1)).toBe(true);
    expect(eitherPositive(-1, 1)).toBe(true);
    expect(eitherPositive(-1, -1)).toBe(false);
  });
});
