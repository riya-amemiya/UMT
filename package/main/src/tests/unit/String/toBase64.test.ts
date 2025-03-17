import { toBase64 } from "@/String/toBase64";

describe("toBase64", () => {
  it("should convert normal string to Base64", () => {
    expect(toBase64("test")).toBe("dGVzdA==");
  });

  it("should convert empty string to Base64", () => {
    expect(toBase64("")).toBe("");
  });

  it("should convert string with special characters to Base64", () => {
    expect(toBase64("@#%")).toBe("QCMl");
  });

  it("should convert string with Japanese characters to Base64", () => {
    expect(toBase64("あいうえお")).toBe("44GC44GE44GG44GI44GK");
  });

  it("should convert long string to Base64", () => {
    expect(
      toBase64("This is a long string to test the Base64 conversion"),
    ).toBe(
      "VGhpcyBpcyBhIGxvbmcgc3RyaW5nIHRvIHRlc3QgdGhlIEJhc2U2NCBjb252ZXJzaW9u",
    );
  });
});
