import { encodeHex } from "@/Crypto/encodeHex";

describe("encodeHex", () => {
  test("encodes a string as UTF-8", () => {
    expect(encodeHex("Hello")).toBe("48656c6c6f");
  });

  test("encodes multi-byte characters", () => {
    expect(encodeHex("\u{3042}")).toBe("e38182");
    expect(encodeHex("\u{1F600}")).toBe("f09f9880");
  });

  test("encodes a Uint8Array with lowercase digits and zero padding", () => {
    expect(encodeHex(new Uint8Array([0, 1, 15, 16, 171, 255]))).toBe(
      "00010f10abff",
    );
  });

  test("encodes empty input", () => {
    expect(encodeHex("")).toBe("");
    expect(encodeHex(new Uint8Array())).toBe("");
  });
});
