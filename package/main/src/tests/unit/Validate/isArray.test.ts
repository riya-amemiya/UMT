import { isArray } from "@/Validate/isArray";

describe("isArray", () => {
  it("should return true for an array", () => {
    expect(isArray<number>([1, 2, 3])).toBe(true);
    expect(isArray<string>(["a", "b", "c"])).toBe(true);
    expect(isArray<boolean>([true, false])).toBe(true);
    expect(isArray<object>([{ a: 1 }, { b: 2 }])).toBe(true);
    expect(isArray<never>([])).toBe(true);
  });

  it("should work as a type guard", () => {
    const value: unknown = [1, 2, 3];
    if (isArray<number>(value)) {
      // TypeScript should recognize value as number[] here
      const sum = value.reduce((a, b) => a + b, 0);
      expect(sum).toBe(6);
    }
  });

  it("should return false for a non-array value", () => {
    expect(isArray<number>(1)).toBe(false);
    expect(isArray<string>("hello")).toBe(false);
    expect(isArray<boolean>(true)).toBe(false);
    expect(isArray<object>({ a: 1 })).toBe(false);
    expect(isArray<null>(null)).toBe(false);
    expect(isArray<undefined>(undefined)).toBe(false);
  });
});
