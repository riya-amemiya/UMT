import type { FormatOptions } from "$/string/formatString/formatOptions";

describe("FormatOptions", () => {
  it("FormatOptions without fields", () => {
    const _: FormatOptions = {};
    expect(_).toEqual(_);
  });
  it("FormatOptions with formatters", () => {
    const _: FormatOptions = {
      formatters: { upper: (value) => String(value).toUpperCase() },
    };
    expect(_).toEqual(_);
  });
  it("FormatOptions with locale", () => {
    const _: FormatOptions = { locale: "ja-JP" };
    expect(_).toEqual(_);
  });
});
