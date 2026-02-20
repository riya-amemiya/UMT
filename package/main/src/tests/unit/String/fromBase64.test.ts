import { toBase64 } from "@/String";
import { fromBase64 } from "@/String/fromBase64";

describe("fromBase64", () => {
  it("should convert Base64 to normal string", () => {
    expect(fromBase64("dGVzdA==")).toBe("test");
    expect(fromBase64(toBase64("test"))).toBe("test");
  });

  it("should convert empty Base64 string", () => {
    expect(fromBase64("")).toBe("");
  });

  it("should convert Base64 string containing special characters", () => {
    expect(fromBase64("QCMl")).toBe("@#%");
    expect(fromBase64(toBase64("@#%"))).toBe("@#%");
  });

  it("should convert Base64 string containing Japanese characters", () => {
    expect(fromBase64("44GC44GE44GG44GI44GK")).toBe("あいうえお");
    expect(fromBase64(toBase64("あいうえお"))).toBe("あいうえお");
  });

  it("should convert Base64 string containing emojis", () => {
    expect(fromBase64(toBase64("🌊🌍🌎"))).toBe("🌊🌍🌎");
  });

  it("should handle different Base64 padding patterns", () => {
    expect(fromBase64("YQ==")).toBe("a"); // 2 padding chars
    expect(fromBase64("YWE=")).toBe("aa"); // 1 padding char
    expect(fromBase64("YWFh")).toBe("aaa"); // no padding
  });
});
