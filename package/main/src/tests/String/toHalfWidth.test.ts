import { toHalfWidth } from "@/String/toHalfWidth";

describe("toHalfWidth", () => {
  it("全角の英数字を半角に変換する", () => {
    expect(toHalfWidth("０１２３４５６７８９")).toBe("0123456789");
    expect(toHalfWidth("ＡＢＣＤＥＦＧＨＩＪ")).toBe("ABCDEFGHIJ");
    expect(toHalfWidth("ａｂｃｄｅｆｇｈｉｊ")).toBe("abcdefghij");
  });

  it("全角と半角が混在する文字列を正しく変換する", () => {
    expect(toHalfWidth("ＡＢＣabc１２３123")).toBe("ABCabc123123");
  });

  it("変換対象外の文字はそのままにする", () => {
    expect(toHalfWidth("漢字カタカナ、。")).toBe("漢字カタカナ、。");
  });
});
