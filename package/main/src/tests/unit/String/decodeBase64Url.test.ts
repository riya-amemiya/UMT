import { decodeBase64Url } from "@/String/decodeBase64Url";
import { encodeBase64Url } from "@/String/encodeBase64Url";

describe("decodeBase64Url", () => {
  test("decodes input without padding", () => {
    expect(decodeBase64Url("SGVsbG8gV29ybGQ")).toBe("Hello World");
    expect(decodeBase64Url("Zg")).toBe("f");
    expect(decodeBase64Url("Zm8")).toBe("fo");
    expect(decodeBase64Url("Zm9v")).toBe("foo");
  });

  test("decodes input with padding", () => {
    expect(decodeBase64Url("SGVsbG8gV29ybGQ=")).toBe("Hello World");
    expect(decodeBase64Url("Zg==")).toBe("f");
  });

  test("decodes the URL-safe alphabet", () => {
    expect(decodeBase64Url("w7_Dvg")).toBe("\u{FF}\u{FE}");
    expect(decodeBase64Url("8J-YgD8-")).toBe("\u{1F600}?>");
  });

  test("decodes an empty string", () => {
    expect(decodeBase64Url("")).toBe("");
  });

  test("round-trips with encodeBase64Url", () => {
    const value = "\u{3053}\u{3093}\u{306B}\u{3061}\u{306F} ~?>/+";
    expect(decodeBase64Url(encodeBase64Url(value))).toBe(value);
  });
});
