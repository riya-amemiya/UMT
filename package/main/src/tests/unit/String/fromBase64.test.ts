import { toBase64 } from "@/String";
import { fromBase64 } from "@/String/fromBase64";

describe("fromBase64", () => {
  it("Base64から通常の文字列に変換", () => {
    expect(fromBase64("dGVzdA==")).toBe("test");
    expect(fromBase64(toBase64("test"))).toBe("test");
  });

  it("空のBase64文字列を変換", () => {
    expect(fromBase64("")).toBe("");
    expect(fromBase64(toBase64(""))).toBe("");
  });

  it("特殊文字を含むBase64文字列を変換", () => {
    expect(fromBase64("QCMl")).toBe("@#%");
    expect(fromBase64(toBase64("@#%"))).toBe("@#%");
  });

  it("日本語を含むBase64文字列を変換", () => {
    expect(fromBase64("44GC44GE44GG44GI44GK")).toBe("あいうえお");
    expect(fromBase64(toBase64("あいうえお"))).toBe("あいうえお");
  });

  it("長いBase64文字列を変換", () => {
    expect(
      fromBase64(
        "VGhpcyBpcyBhIGxvbmcgc3RyaW5nIHRvIHRlc3QgdGhlIEJhc2U2NCBjb252ZXJzaW9u",
      ),
    ).toBe("This is a long string to test the Base64 conversion");
    expect(
      fromBase64(
        toBase64("This is a long string to test the Base64 conversion"),
      ),
    ).toBe("This is a long string to test the Base64 conversion");
  });
});
