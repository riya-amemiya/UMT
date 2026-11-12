import { decodeHex } from "@/Crypto/decodeHex";
import { encodeHex } from "@/Crypto/encodeHex";

describe("decodeHex", () => {
  test("decodes lowercase hex", () => {
    expect(decodeHex("48656c6c6f")).toEqual(
      new Uint8Array([72, 101, 108, 108, 111]),
    );
  });

  test("decodes uppercase hex", () => {
    expect(decodeHex("00010F10ABFF")).toEqual(
      new Uint8Array([0, 1, 15, 16, 171, 255]),
    );
  });

  test("decodes an empty string", () => {
    expect(decodeHex("")).toEqual(new Uint8Array());
  });

  test("round-trips with encodeHex", () => {
    const bytes = new Uint8Array(Array.from({ length: 256 }, (_, i) => i));
    expect(decodeHex(encodeHex(bytes))).toEqual(bytes);
  });
});
