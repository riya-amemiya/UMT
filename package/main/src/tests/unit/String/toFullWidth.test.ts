import { toFullWidth } from "@/String/toFullWidth";

describe("toFullWidth", () => {
  it("should convert half-width digits to full-width", () => {
    expect(toFullWidth("123")).toBe("１２３");
  });

  it("should convert half-width uppercase letters to full-width", () => {
    expect(toFullWidth("ABC")).toBe("ＡＢＣ");
  });

  it("should convert half-width lowercase letters to full-width", () => {
    expect(toFullWidth("abc")).toBe("ａｂｃ");
  });

  it("should convert mixed alphanumeric characters", () => {
    expect(toFullWidth("Abc123")).toBe("Ａｂｃ１２３");
  });

  it("should leave non-alphanumeric characters unchanged", () => {
    expect(toFullWidth("a-b!")).toBe("ａ-ｂ!");
  });

  it("should handle empty string", () => {
    expect(toFullWidth("")).toBe("");
  });

  it("should leave already-full-width alphanumeric unchanged", () => {
    expect(toFullWidth("Ａｂｃ１２３")).toBe("Ａｂｃ１２３");
  });

  it("should convert mixed half-width and full-width input", () => {
    expect(toFullWidth("ABCabc１２３123")).toBe("ＡＢＣａｂｃ１２３１２３");
  });

  it("should leave CJK, punctuation, and spaces unchanged", () => {
    expect(toFullWidth("漢字カタカナ、。")).toBe("漢字カタカナ、。");
    expect(toFullWidth("Hello 世界 123")).toBe("Ｈｅｌｌｏ 世界 １２３");
    expect(toFullWidth("!@# $%^")).toBe("!@# $%^");
    expect(toFullWidth("   ")).toBe("   ");
  });

  it("should convert every ASCII alphanumeric by 0xFEE0", () => {
    const asciiAlphanumeric =
      "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    const expected = Array.from(asciiAlphanumeric, (character) =>
      String.fromCodePoint((character.codePointAt(0) ?? 0) + 0xfe_e0),
    ).join("");
    expect(toFullWidth(asciiAlphanumeric)).toBe(expected);
  });

  it("should treat a surrogate pair as a single non-alphanumeric character", () => {
    expect(toFullWidth("A😀b")).toBe("Ａ😀ｂ");
  });
});
