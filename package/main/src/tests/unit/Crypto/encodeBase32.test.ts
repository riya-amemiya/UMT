import { decodeBase32 } from "@/Crypto/decodeBase32";
import { encodeBase32 } from "@/Crypto/encodeBase32";

describe("encodeBase32", () => {
  test("encodes a simple string", () => {
    expect(encodeBase32("Hello")).toBe("JBSWY3DP");
  });

  test("encodes an empty string", () => {
    expect(encodeBase32("")).toBe("");
  });

  test("encodes an empty Uint8Array", () => {
    expect(encodeBase32(new Uint8Array())).toBe("");
  });

  test("encodes a string with padding", () => {
    expect(encodeBase32("f")).toBe("MY======");
    expect(encodeBase32("fo")).toBe("MZXQ====");
    expect(encodeBase32("foo")).toBe("MZXW6===");
    expect(encodeBase32("foob")).toBe("MZXW6YQ=");
    expect(encodeBase32("fooba")).toBe("MZXW6YTB");
    expect(encodeBase32("foobar")).toBe("MZXW6YTBOI======");
  });

  test("encodes a Uint8Array", () => {
    const bytes = new Uint8Array([72, 101, 108, 108, 111]); // "Hello"
    expect(encodeBase32(bytes)).toBe("JBSWY3DP");
  });

  test("encodes a Uint8Array of zeros", () => {
    expect(encodeBase32(new Uint8Array(1))).toBe("AA======");
    expect(encodeBase32(new Uint8Array(2))).toBe("AAAA====");
    expect(encodeBase32(new Uint8Array(3))).toBe("AAAAA===");
  });

  test("encodes special characters", () => {
    expect(encodeBase32("こんにちは")).toBe("4OAZHY4CSPRYDK7DQGQ6HANP");
  });

  test("encodes longer text", () => {
    const text = "The quick brown fox jumps over the lazy dog";
    expect(encodeBase32(text)).toBe(
      "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWO===",
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
      expect(decoder.decode(decodeBase32(encodeBase32(sample)))).toBe(sample);
    }
  });

  test("round-trips every 1-byte value", () => {
    for (let value = 0; value < 256; value++) {
      const bytes = new Uint8Array([value]);
      expect(Array.from(decodeBase32(encodeBase32(bytes)))).toEqual(
        Array.from(bytes),
      );
    }
  });

  test("string and Uint8Array encodings match for UTF-8 input", () => {
    const text = "XMLHttpRequest こんにちは";
    expect(encodeBase32(text)).toBe(
      encodeBase32(new TextEncoder().encode(text)),
    );
  });
});
