import { encodeBase32 } from "@/Crypto/encodeBase32";

describe("encodeBase32", () => {
  test("encodes a simple string", () => {
    expect(encodeBase32("Hello")).toBe("JBSWY3DP");
  });

  test("encodes an empty string", () => {
    expect(encodeBase32("")).toBe("");
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

  test("encodes special characters", () => {
    expect(encodeBase32("こんにちは")).toBe("4OAZHY4CSPRYDK7DQGQ6HANP");
  });

  test("encodes longer text", () => {
    const text = "The quick brown fox jumps over the lazy dog";
    expect(encodeBase32(text)).toBe(
      "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWO===",
    );
  });
});
