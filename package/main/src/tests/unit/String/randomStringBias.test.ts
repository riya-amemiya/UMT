import { randomString } from "@/String/randomString";

describe("randomString bias check", () => {
  it("should generate correct length strings for various charset lengths", () => {
    // Test with charset length 1
    // 2^32 % 1 = 0. No rejections.
    expect(randomString(10, "a")).toBe("aaaaaaaaaa");

    // Test with charset length 2
    // 2^32 % 2 = 0. No rejections.
    expect(randomString(10, "ab")).toMatch(/^[ab]{10}$/);

    // Test with charset length 3
    // 2^32 % 3 = 1. One rejection possible (value 4294967295).
    expect(randomString(100, "abc")).toMatch(/^[abc]{100}$/);

    // Test with charset length 62 (standard)
    expect(randomString(100)).toHaveLength(100);
  });

  it("should handle large charset (e.g. 256 chars)", () => {
    let chars = "";
    for (let index = 0; index < 256; index++) {
      chars += String.fromCharCode(index);
    }
    const str = randomString(100, chars);
    expect(str).toHaveLength(100);
  });
});
