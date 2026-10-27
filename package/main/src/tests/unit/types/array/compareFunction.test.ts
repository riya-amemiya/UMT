import type { CompareFunction } from "$/array/compareFunction";

describe("CompareFunction", () => {
  it("CompareFunction<number>", () => {
    const _: CompareFunction<number> = (a, b) => a - b;
    expect(_(2, 1)).toBe(1);
  });
  it("CompareFunction<string>", () => {
    const _: CompareFunction<string> = (a, b) => a.localeCompare(b);
    expect(_("a", "b")).toBe(-1);
  });
});
