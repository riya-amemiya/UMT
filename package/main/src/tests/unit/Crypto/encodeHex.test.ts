import { decodeHex } from "@/Crypto/decodeHex";
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

  test("encodes a Uint8Array of zeros", () => {
    expect(encodeHex(new Uint8Array(1))).toBe("00");
    expect(encodeHex(new Uint8Array(2))).toBe("0000");
    expect(encodeHex(new Uint8Array(3))).toBe("000000");
  });

  test("encodes longer text", () => {
    expect(encodeHex("The quick brown fox jumps over the lazy dog")).toBe(
      "54686520717569636b2062726f776e20666f78206a756d7073206f76657220746865206c617a7920646f67",
    );
  });

  test("encodes CJK and mixed emoji text", () => {
    expect(encodeHex("こんにちは")).toBe("e38193e38293e381abe381a1e381af");
    expect(encodeHex("A😀b")).toBe("41f09f988062");
    expect(encodeHex("Hello 👋 世界")).toBe(
      "48656c6c6f20f09f918b20e4b896e7958c",
    );
  });

  test("round-trips ASCII, CJK, emoji, and mixed text", () => {
    const samples = [
      "Hello",
      "The quick brown fox jumps over the lazy dog",
      "こんにちは",
      "A😀b",
      "Hello 👋 世界",
      "a".repeat(256),
    ];
    const decoder = new TextDecoder();
    for (const sample of samples) {
      expect(decoder.decode(decodeHex(encodeHex(sample)))).toBe(sample);
    }
  });

  test("round-trips every 1-byte value", () => {
    for (let value = 0; value < 256; value++) {
      const bytes = new Uint8Array([value]);
      expect(Array.from(decodeHex(encodeHex(bytes)))).toEqual(
        Array.from(bytes),
      );
    }
  });

  test("string and Uint8Array encodings match for UTF-8 input", () => {
    const text = "XMLHttpRequest こんにちは";
    expect(encodeHex(text)).toBe(encodeHex(new TextEncoder().encode(text)));
  });

  test("matches the previous toString+padStart+join encoding", () => {
    const samples: Array<string | Uint8Array> = [
      "",
      "Hello",
      "f",
      "foobar",
      "The quick brown fox jumps over the lazy dog",
      "こんにちは世界",
      "A😀b",
      "a".repeat(256),
      new Uint8Array(),
      new Uint8Array([0, 1, 15, 16, 171, 255]),
      new Uint8Array(256).map((_, index) => index),
    ];
    for (const sample of samples) {
      const bytes =
        typeof sample === "string" ? new TextEncoder().encode(sample) : sample;
      const previous = Array.from(bytes, (byte) =>
        byte.toString(16).padStart(2, "0"),
      ).join("");
      expect(encodeHex(sample)).toBe(previous);
    }
  });
});
