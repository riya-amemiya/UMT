import { isNotNullish } from "@/Predicate/isNotNullish";

describe("isNotNullish", () => {
  it("returns false for null", () => {
    expect(isNotNullish(null)).toBe(false);
  });

  it("returns false for undefined", () => {
    expect(isNotNullish(undefined)).toBe(false);
  });

  it("returns true for 0", () => {
    expect(isNotNullish(0)).toBe(true);
  });

  it("returns true for empty string", () => {
    expect(isNotNullish("")).toBe(true);
  });

  it("returns true for false", () => {
    expect(isNotNullish(false)).toBe(true);
  });

  it("returns true for NaN", () => {
    expect(isNotNullish(Number.NaN)).toBe(true);
  });

  it("returns true for objects", () => {
    expect(isNotNullish({})).toBe(true);
    expect(isNotNullish([])).toBe(true);
  });

  it("returns true for functions", () => {
    expect(
      isNotNullish(() => {
        /* intentionally empty */
      }),
    ).toBe(true);
  });

  it("narrows type correctly", () => {
    const value: string = "hello";
    if (isNotNullish(value)) {
      const _str: string = value;
      expect(_str).toBe("hello");
    }
  });
});
