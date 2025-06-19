import { bitwise } from "@/Math/bitwise";

describe("bitwise", () => {
  it("should rotate 8 bits to the left", () => {
    const result = bitwise(0x12345678, 8);
    expect(result.toString(16)).toBe("34567812");
  });

  it("should rotate 16 bits to the left", () => {
    const result = bitwise(0x12345678, 16);
    expect(result.toString(16)).toBe("56781234");
  });

  it("should rotate 24 bits to the left", () => {
    const result = bitwise(0x12345678, 24);
    expect(result.toString(16)).toBe("78123456");
  });

  it("should rotate 32 bits to the left (same as original value)", () => {
    const result = bitwise(0x12345678, 32);
    expect(result.toString(16)).toBe("12345678");
  });

  it("should rotate 0 bits to the left (same as original value)", () => {
    const result = bitwise(0x12345678, 0);
    expect(result.toString(16)).toBe("12345678");
  });

  it("should handle negative bit count (same as right rotation)", () => {
    const result = bitwise(0x12345678, -8);
    expect(result.toString(16)).toBe("78123456");
  });

  it("should handle bit count over 32 (rotate by remainder when divided by 32)", () => {
    const result = bitwise(0x12345678, 40); // 40 % 32 = 8
    expect(result.toString(16)).toBe("34567812");
  });

  it("should rotate 8 bits to the right", () => {
    const result = bitwise(0x12345678, 8, "right");
    expect(result.toString(16)).toBe("78123456");
  });

  it("should rotate 16 bits to the right", () => {
    const result = bitwise(0x12345678, 16, "right");
    expect(result.toString(16)).toBe("56781234");
  });

  it("should rotate 24 bits to the right", () => {
    const result = bitwise(0x12345678, 24, "right");
    expect(result.toString(16)).toBe("34567812");
  });

  it("should rotate 32 bits to the right (same as original value)", () => {
    const result = bitwise(0x12345678, 32, "right");
    expect(result.toString(16)).toBe("12345678");
  });

  it("should rotate 0 bits to the right (same as original value)", () => {
    const result = bitwise(0x12345678, 0, "right");
    expect(result.toString(16)).toBe("12345678");
  });

  it("should handle bit count over 32 (rotate by remainder when divided by 32)", () => {
    const result = bitwise(0x12345678, 40, "right"); // 40 % 32 = 8
    expect(result.toString(16)).toBe("78123456");
  });

  it("should throw error for invalid direction", () => {
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    expect(() => bitwise(0x12345678, 8, "invalid" as any)).toThrowError(
      "Invalid direction",
    );
  });
});
