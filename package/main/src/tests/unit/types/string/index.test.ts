import type { FormatValue } from "$/string/index";

describe("string index", () => {
  it("FormatValue", () => {
    const _: FormatValue[] = [
      "a",
      1,
      true,
      new Date(0),
      null,
      undefined,
      { a: 1 },
      [1],
    ];
    expect(_).toEqual(_);
  });
  it("symbol extends FormatValue", () => {
    const _: symbol extends FormatValue ? true : false = false;
    expect(_).toEqual(_);
  });
});
