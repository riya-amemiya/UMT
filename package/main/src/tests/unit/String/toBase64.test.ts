import { toBase64 } from "@/String/toBase64";

describe("toBase64", () => {
  it("通常の文字列をBase64に変換", () => {
    expect(toBase64("test")).toBe("dGVzdA==");
  });

  it("空の文字列をBase64に変換", () => {
    expect(toBase64("")).toBe("");
  });

  it("特殊文字を含む文字列をBase64に変換", () => {
    expect(toBase64("@#%")).toBe("QCMl");
  });

  it("日本語を含む文字列をBase64に変換", () => {
    expect(toBase64("あいうえお")).toBe("44GC44GE44GG44GI44GK");
  });

  it("長い文字列をBase64に変換", () => {
    expect(
      toBase64("This is a long string to test the Base64 conversion"),
    ).toBe(
      "VGhpcyBpcyBhIGxvbmcgc3RyaW5nIHRvIHRlc3QgdGhlIEJhc2U2NCBjb252ZXJzaW9u",
    );
  });
});
