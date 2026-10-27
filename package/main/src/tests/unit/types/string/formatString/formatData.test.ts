import type { FormatData } from "$/string/formatString/formatData";

describe("FormatData", () => {
  it("FormatData as Record<string, unknown>", () => {
    const _: FormatData = { name: "Alice", age: 25 };
    expect(_).toEqual(_);
  });
  it("FormatData as FormatValue[]", () => {
    const _: FormatData = ["Alice", 25];
    expect(_).toEqual(_);
  });
  it("string extends FormatData", () => {
    const _: string extends FormatData ? true : false = false;
    expect(_).toEqual(_);
  });
});
