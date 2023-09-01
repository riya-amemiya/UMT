import { isArr } from "@/Tool/isArr";

describe("isArr", () => {
  it("should return true for an array", () => {
    expect(isArr<number>([1, 2, 3])).toBe(true);
    expect(isArr<string>(["a", "b", "c"])).toBe(true);
    expect(isArr<boolean>([true, false])).toBe(true);
    expect(isArr<object>([{ a: 1 }, { b: 2 }])).toBe(true);
  });

  it("should return false for a non-array value", () => {
    expect(isArr<number>(1)).toBe(false);
    expect(isArr<string>("hello")).toBe(false);
    expect(isArr<boolean>(true)).toBe(false);
    expect(isArr<object>({ a: 1 })).toBe(false);
    expect(isArr<null>(null)).toBe(false);
    expect(isArr<undefined>(undefined)).toBe(false);
  });
});
