import { isNullish } from "@/Predicate/isNullish";

describe("isNullish", () => {
  it("returns true for null", () => {
    expect(isNullish(null)).toBe(true);
  });

  it("returns true for undefined", () => {
    expect(isNullish(undefined)).toBe(true);
  });

  it("returns false for 0", () => {
    expect(isNullish(0)).toBe(false);
  });

  it("returns false for empty string", () => {
    expect(isNullish("")).toBe(false);
  });

  it("returns false for false", () => {
    expect(isNullish(false)).toBe(false);
  });

  it("returns false for NaN", () => {
    expect(isNullish(Number.NaN)).toBe(false);
  });

  it("returns false for objects", () => {
    expect(isNullish({})).toBe(false);
    expect(isNullish([])).toBe(false);
  });

  it("returns false for functions", () => {
    expect(
      isNullish(() => {
        /* intentionally empty */
      }),
    ).toBe(false);
  });

  it("narrows type correctly", () => {
    const value: string | null | undefined = "hello";
    if (!isNullish(value)) {
      const _str: string = value;
      expect(_str).toBe("hello");
    }
  });
});
