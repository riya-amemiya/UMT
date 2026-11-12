import { wrapText } from "@/String/wrapText";

describe("wrapText", () => {
  it("should wrap according to JSDoc examples", () => {
    expect(wrapText("The quick brown fox", 10)).toBe("The quick\nbrown fox");
    expect(wrapText("a extraordinary b", 5)).toBe("a\nextraordinary\nb");
  });

  it("should keep a line that is exactly width long", () => {
    expect(wrapText("ab cd", 5)).toBe("ab cd");
    expect(wrapText("ab cd", 4)).toBe("ab\ncd");
  });

  it("should collapse runs of separators", () => {
    expect(wrapText("  a \t b\f\vc  ", 10)).toBe("a b c");
  });

  it("should keep existing line breaks and empty lines", () => {
    expect(wrapText("aaa bbb\nccc", 3)).toBe("aaa\nbbb\nccc");
    expect(wrapText("a\n\nb", 5)).toBe("a\n\nb");
    expect(wrapText("ab\r\ncd", 10)).toBe("ab\ncd");
  });

  it("should put every word on its own line when width is zero", () => {
    expect(wrapText("a b", 0)).toBe("a\nb");
  });

  it("should return an empty string for empty or blank input", () => {
    expect(wrapText("", 10)).toBe("");
    expect(wrapText("   ", 10)).toBe("");
  });

  it("should measure width in UTF-16 code units", () => {
    expect(wrapText("\u{1f600} a", 3)).toBe("\u{1f600}\na");
  });
});
