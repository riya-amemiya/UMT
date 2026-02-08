import { every } from "@/Predicate/every";

describe("every", () => {
  it("returns true when all predicates pass", () => {
    const isPositiveEven = every(
      (n: number) => n > 0,
      (n: number) => n % 2 === 0,
    );
    expect(isPositiveEven(4)).toBe(true);
    expect(isPositiveEven(8)).toBe(true);
  });

  it("returns false when any predicate fails", () => {
    const isPositiveEven = every(
      (n: number) => n > 0,
      (n: number) => n % 2 === 0,
    );
    expect(isPositiveEven(-2)).toBe(false);
    expect(isPositiveEven(3)).toBe(false);
  });

  it("short-circuits on first failure", () => {
    let secondCalled = false;
    const combined = every(
      () => false,
      () => {
        secondCalled = true;
        return true;
      },
    );
    combined();
    expect(secondCalled).toBe(false);
  });

  it("handles single predicate", () => {
    const single = every((n: number) => n > 0);
    expect(single(1)).toBe(true);
    expect(single(-1)).toBe(false);
  });

  it("returns true for no predicates", () => {
    const always = every();
    expect(always()).toBe(true);
  });

  it("handles multiple arguments", () => {
    const bothPositive = every(
      (a: number, _b: number) => a > 0,
      (_a: number, b: number) => b > 0,
    );
    expect(bothPositive(1, 2)).toBe(true);
    expect(bothPositive(1, -1)).toBe(false);
    expect(bothPositive(-1, 1)).toBe(false);
  });
});
