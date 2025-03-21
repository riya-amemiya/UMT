import { isNotEmpty } from "@/Validate/isNotEmpty";

describe("isNotEmpty", () => {
  it("should return true for a non-empty object", () => {
    expect(isNotEmpty({ a: 1 })).toBe(true);
    expect(isNotEmpty({ a: 1, b: 2 })).toBe(true);
    expect(isNotEmpty({ a: { b: 2 } })).toBe(true);
  });

  it("should return false for an empty object", () => {
    expect(isNotEmpty({})).toBe(false);
  });

  it("should return true for a non-empty array", () => {
    expect(isNotEmpty([1, 2, 3])).toBe(true);
    expect(isNotEmpty(["a", "b", "c"])).toBe(true);
    expect(isNotEmpty([true, false])).toBe(true);
    expect(isNotEmpty([{ a: 1 }, { b: 2 }])).toBe(true);
  });

  it("should return false for an empty array", () => {
    expect(isNotEmpty([])).toBe(false);
  });

  // Note: Map and Set are always considered empty by Object.keys()
  it("should return false for Map and Set (regardless of content)", () => {
    expect(
      isNotEmpty(
        new Map([
          ["a", 1],
          ["b", 2],
        ]),
      ),
    ).toBe(false);
    expect(isNotEmpty(new Set([1, 2, 3]))).toBe(false);
    expect(isNotEmpty(new Map())).toBe(false);
    expect(isNotEmpty(new Set())).toBe(false);
  });
});
