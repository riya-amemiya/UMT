import { toHalfWidth } from "@/String/toHalfWidth";

describe("toHalfWidth", () => {
  it("should convert full-width alphanumeric characters to half-width", () => {
    expect(toHalfWidth("０１２３４５６７８９")).toBe("0123456789");
    expect(toHalfWidth("ＡＢＣＤＥＦＧＨＩＪ")).toBe("ABCDEFGHIJ");
    expect(toHalfWidth("ａｂｃｄｅｆｇｈｉｊ")).toBe("abcdefghij");
  });

  it("should correctly convert strings containing both full-width and half-width characters", () => {
    expect(toHalfWidth("ＡＢＣabc１２３123")).toBe("ABCabc123123");
  });

  it("should keep non-target characters unchanged", () => {
    expect(toHalfWidth("漢字カタカナ、。")).toBe("漢字カタカナ、。");
  });

  it("should handle empty string", () => {
    expect(toHalfWidth("")).toBe("");
  });

  it("should leave already-half-width alphanumeric unchanged", () => {
    expect(toHalfWidth("Abc123")).toBe("Abc123");
  });

  it("should convert mixed full-width and half-width input", () => {
    expect(toHalfWidth("ＡＢＣａｂｃ１２３123")).toBe("ABCabc123123");
  });

  it("should leave CJK, punctuation, and spaces unchanged", () => {
    expect(toHalfWidth("漢字カタカナ、。")).toBe("漢字カタカナ、。");
    expect(toHalfWidth("Ｈｅｌｌｏ 世界 １２３")).toBe("Hello 世界 123");
    expect(toHalfWidth("！＠＃ ＄％＾")).toBe("！＠＃ ＄％＾");
    expect(toHalfWidth("　")).toBe("　");
  });

  it("should convert every full-width alphanumeric by 0xFEE0", () => {
    const asciiAlphanumeric =
      "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    const fullWidthAlphanumeric = Array.from(asciiAlphanumeric, (character) =>
      String.fromCodePoint((character.codePointAt(0) ?? 0) + 0xfe_e0),
    ).join("");
    expect(toHalfWidth(fullWidthAlphanumeric)).toBe(asciiAlphanumeric);
  });

  it("should treat a surrogate pair as a single non-alphanumeric character", () => {
    expect(toHalfWidth("Ａ😀ｂ")).toBe("A😀b");
  });
});
