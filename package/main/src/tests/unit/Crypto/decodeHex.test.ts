import { decodeHex } from "@/Crypto/decodeHex";
import { encodeHex } from "@/Crypto/encodeHex";

const previousDecodeHex = (input: string): Uint8Array =>
  Uint8Array.from({ length: input.length >> 1 }, (_, index) =>
    Number.parseInt(input.slice(index * 2, index * 2 + 2), 16),
  );

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

  test("decodes mixed-case hex", () => {
    expect(decodeHex("AbCdEf")).toEqual(new Uint8Array([171, 205, 239]));
  });

  test("decodes an empty string", () => {
    expect(decodeHex("")).toEqual(new Uint8Array());
  });

  test("decodes a single byte and zero bytes", () => {
    expect(decodeHex("00")).toEqual(new Uint8Array([0]));
    expect(decodeHex("ff")).toEqual(new Uint8Array([255]));
    expect(decodeHex("FF")).toEqual(new Uint8Array([255]));
  });

  test("decodes longer text", () => {
    expect(
      new TextDecoder().decode(
        decodeHex(
          "54686520717569636b2062726f776e20666f78206a756d7073206f76657220746865206c617a7920646f67",
        ),
      ),
    ).toBe("The quick brown fox jumps over the lazy dog");
  });

  test("decodes CJK and mixed emoji text", () => {
    expect(
      new TextDecoder().decode(decodeHex("e38193e38293e381abe381a1e381af")),
    ).toBe("こんにちは");
    expect(new TextDecoder().decode(decodeHex("41f09f988062"))).toBe("A😀b");
    expect(
      new TextDecoder().decode(decodeHex("48656c6c6f20f09f918b20e4b896e7958c")),
    ).toBe("Hello 👋 世界");
  });

  test("drops the last character of an odd-length string", () => {
    expect(decodeHex("abc")).toEqual(new Uint8Array([171]));
    expect(decodeHex("a")).toEqual(new Uint8Array());
    expect(decodeHex("F")).toEqual(new Uint8Array());
  });

  test("maps non-hex pairs to 0", () => {
    expect(decodeHex("zz")).toEqual(new Uint8Array([0]));
    expect(decodeHex("gg")).toEqual(new Uint8Array([0]));
    expect(decodeHex("GG")).toEqual(new Uint8Array([0]));
  });

  test("maps non-ASCII pairs to 0", () => {
    expect(decodeHex("\u{1F600}")).toEqual(new Uint8Array([0]));
    expect(decodeHex("ああ")).toEqual(new Uint8Array([0]));
  });

  test("round-trips with encodeHex", () => {
    const bytes = new Uint8Array(Array.from({ length: 256 }, (_, i) => i));
    expect(decodeHex(encodeHex(bytes))).toEqual(bytes);
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

  test("matches the previous parseInt+slice decoding for valid hex", () => {
    const samples = [
      "",
      "00",
      "ff",
      "FF",
      "48656c6c6f",
      "00010F10ABFF",
      "AbCdEf",
      "abc",
      "zz",
      "a".repeat(512),
      encodeHex("The quick brown fox jumps over the lazy dog"),
      encodeHex("こんにちは世界"),
      encodeHex("A😀b"),
      encodeHex(new Uint8Array(Array.from({ length: 256 }, (_, i) => i))),
    ];
    for (const sample of samples) {
      expect(Array.from(decodeHex(sample))).toEqual(
        Array.from(previousDecodeHex(sample)),
      );
    }
  });
});
