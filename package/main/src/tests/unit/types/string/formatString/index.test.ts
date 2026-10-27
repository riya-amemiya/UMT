import type {
  FormatData,
  FormatOptions,
  Formatter,
  FormatValue,
} from "$/string/formatString/index";

describe("formatString index", () => {
  it("FormatData", () => {
    const _: FormatData = ["a"];
    expect(_).toEqual(_);
  });
  it("FormatOptions", () => {
    const _: FormatOptions = { locale: "ja-JP" };
    expect(_).toEqual(_);
  });
  it("Formatter", () => {
    const _: Formatter = (value) => String(value);
    expect(_(1)).toBe("1");
  });
  it("FormatValue", () => {
    const _: FormatValue = "a";
    expect(_).toEqual(_);
  });
});
