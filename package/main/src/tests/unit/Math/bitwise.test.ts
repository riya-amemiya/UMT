import { bitwise } from "@/Math/bitwise";

describe("bitwise", () => {
  it("左に8ビット回転する", () => {
    const result = bitwise(0x12345678, 8);
    expect(result.toString(16)).toBe("34567812");
  });

  it("左に16ビット回転する", () => {
    const result = bitwise(0x12345678, 16);
    expect(result.toString(16)).toBe("56781234");
  });

  it("左に24ビット回転する", () => {
    const result = bitwise(0x12345678, 24);
    expect(result.toString(16)).toBe("78123456");
  });

  it("左に32ビット回転する (元の値と同じ)", () => {
    const result = bitwise(0x12345678, 32);
    expect(result.toString(16)).toBe("12345678");
  });

  it("左に0ビット回転する (元の値と同じ)", () => {
    const result = bitwise(0x12345678, 0);
    expect(result.toString(16)).toBe("12345678");
  });

  it("負のビット数で回転する (右回転と同じ)", () => {
    const result = bitwise(0x12345678, -8);
    expect(result.toString(16)).toBe("78123456");
  });

  it("ビット数が32を超える場合 (32で割った余りのビット数で回転)", () => {
    const result = bitwise(0x12345678, 40); // 40 % 32 = 8
    expect(result.toString(16)).toBe("34567812");
  });

  it("右に8ビット回転する", () => {
    const result = bitwise(0x12345678, 8, "right");
    expect(result.toString(16)).toBe("78123456");
  });

  it("右に16ビット回転する", () => {
    const result = bitwise(0x12345678, 16, "right");
    expect(result.toString(16)).toBe("56781234");
  });

  it("右に24ビット回転する", () => {
    const result = bitwise(0x12345678, 24, "right");
    expect(result.toString(16)).toBe("34567812");
  });

  it("右に32ビット回転する (元の値と同じ)", () => {
    const result = bitwise(0x12345678, 32, "right");
    expect(result.toString(16)).toBe("12345678");
  });

  it("右に0ビット回転する (元の値と同じ)", () => {
    const result = bitwise(0x12345678, 0, "right");
    expect(result.toString(16)).toBe("12345678");
  });

  it("ビット数が32を超える場合 (32で割った余りのビット数で回転)", () => {
    const result = bitwise(0x12345678, 40, "right"); // 40 % 32 = 8
    expect(result.toString(16)).toBe("78123456");
  });

  it("不正な方向を指定する", () => {
    expect(() => bitwise(0x12345678, 8, "invalid" as any)).toThrowError(
      "Invalid direction",
    );
  });
});
