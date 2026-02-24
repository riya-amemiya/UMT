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

  it("should reject values at or above the bias limit", () => {
    // For charset length 3: limit = 4294967296 - (4294967296 % 3) = 4294967295
    // Values >= 4294967295 (i.e., 0xFFFFFFFF) should be rejected
    const original = globalThis.crypto.getRandomValues.bind(globalThis.crypto);
    let callCount = 0;
    const spy = jest
      .spyOn(globalThis.crypto, "getRandomValues")
      .mockImplementation((array: Uint32Array) => {
        callCount++;
        if (callCount === 1) {
          // First call: fill with rejected values followed by valid values
          for (let i = 0; i < array.length; i++) {
            // First value triggers rejection (0xFFFFFFFF >= limit 4294967295)
            // Subsequent values are valid
            array[i] = i === 0 ? 0xffffffff : 0;
          }
        } else {
          // Subsequent calls: all valid
          for (let i = 0; i < array.length; i++) {
            array[i] = i % 3;
          }
        }
        return array;
      });

    const result = randomString(3, "abc");
    expect(result).toHaveLength(3);
    expect(result).toMatch(/^[abc]{3}$/);

    spy.mockRestore();
  });
});
