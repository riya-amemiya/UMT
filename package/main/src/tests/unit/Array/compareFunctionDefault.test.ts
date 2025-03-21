import { compareFunctionDefault } from "@/Array/compareFunctionDefault";

describe("compareFunctionDefault", () => {
  test("should return 1 when first value is greater", () => {
    expect(compareFunctionDefault(2, 1)).toBe(1);
    expect(compareFunctionDefault("b", "a")).toBe(1);
  });

  test("should return -1 when first value is lesser", () => {
    expect(compareFunctionDefault(1, 2)).toBe(-1);
    expect(compareFunctionDefault("a", "b")).toBe(-1);
  });

  test("should return 0 when values are equal", () => {
    expect(compareFunctionDefault(1, 1)).toBe(0);
    expect(compareFunctionDefault("a", "a")).toBe(0);
  });

  test("should work with different types", () => {
    expect(compareFunctionDefault(true, false)).toBe(1);
    expect(compareFunctionDefault(false, true)).toBe(-1);
    expect(compareFunctionDefault(true, true)).toBe(0);
  });

  test("should work with objects that are comparable", () => {
    const date1 = new Date(2024, 0, 1);
    const date2 = new Date(2024, 0, 2);

    expect(compareFunctionDefault(date2, date1)).toBe(1);
    expect(compareFunctionDefault(date1, date2)).toBe(-1);
    expect(compareFunctionDefault(date1, new Date(2024, 0, 1))).toBe(0);
  });
});
