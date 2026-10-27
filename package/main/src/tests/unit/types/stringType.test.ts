import type { FormatData } from "$/string/formatString/formatData";
import type { FormatOptions } from "$/string/formatString/formatOptions";
import type { Formatter } from "$/string/formatString/formatter";
import type { FormatValue } from "$/string/formatString/formatValue";

describe("stringType formatString", () => {
  it("formatValue", () => {
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
  it("formatData", () => {
    const _: FormatData[] = [{ name: "Alice" }, ["Alice", 25]];
    expect(_).toEqual(_);
  });
  it("formatter", () => {
    const _: Formatter = (value, ...arguments_) =>
      `${String(value)}${arguments_.join("")}`;
    expect(_("a", "b", "c")).toBe("abc");
  });
  it("formatOptions", () => {
    const _: FormatOptions[] = [
      {},
      { formatters: { upper: (value) => String(value).toUpperCase() } },
      { locale: "ja-JP" },
    ];
    expect(_).toEqual(_);
  });
});
