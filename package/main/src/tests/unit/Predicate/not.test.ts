import { not } from "@/Predicate/not";

describe("not", () => {
  it("negates a truthy predicate", () => {
    const isEven = (n: number) => n % 2 === 0;
    const isOdd = not(isEven);

    expect(isOdd(1)).toBe(true);
    expect(isOdd(2)).toBe(false);
    expect(isOdd(3)).toBe(true);
    expect(isOdd(4)).toBe(false);
  });

  it("negates a string predicate", () => {
    const isEmpty = (s: string) => s.length === 0;
    const isNotEmpty = not(isEmpty);

    expect(isNotEmpty("")).toBe(false);
    expect(isNotEmpty("hello")).toBe(true);
  });

  it("handles predicates with multiple arguments", () => {
    const greaterThan = (a: number, b: number) => a > b;
    const notGreaterThan = not(greaterThan);

    expect(notGreaterThan(5, 3)).toBe(false);
    expect(notGreaterThan(3, 5)).toBe(true);
    expect(notGreaterThan(3, 3)).toBe(true);
  });

  it("passes through arguments correctly", () => {
    const receivedArgs: unknown[] = [];
    const spy = (...arguments_: unknown[]) => {
      receivedArgs.push(...arguments_);
      return true;
    };
    const negated = not(spy);
    negated("a", "b", "c");
    expect(receivedArgs).toEqual(["a", "b", "c"]);
  });

  it("double negation returns original result", () => {
    const isPositive = (n: number) => n > 0;
    const doubleNot = not(not(isPositive));

    expect(doubleNot(5)).toBe(true);
    expect(doubleNot(-1)).toBe(false);
  });
});
