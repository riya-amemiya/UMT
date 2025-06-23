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
});
