import { isArray } from "@/Tool/isArray";

describe("isArray", () => {
  it("should return true for an array", () => {
    expect(isArray<number>([1, 2, 3])).toBe(true);
    expect(isArray<string>(["a", "b", "c"])).toBe(true);
    expect(isArray<boolean>([true, false])).toBe(true);
    expect(isArray<object>([{ a: 1 }, { b: 2 }])).toBe(true);
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
