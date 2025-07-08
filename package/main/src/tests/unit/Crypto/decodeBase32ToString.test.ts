import { decodeBase32ToString } from "@/Crypto/decodeBase32ToString";

describe("decodeBase32ToString", () => {
  test("decodes a simple string", () => {
    expect(decodeBase32ToString("JBSWY3DP")).toBe("Hello");
  });

  test("decodes an empty string", () => {
    expect(decodeBase32ToString("")).toBe("");
  });

  test("decodes strings with padding", () => {
    expect(decodeBase32ToString("MY======")).toBe("f");
    expect(decodeBase32ToString("MZXQ====")).toBe("fo");
    expect(decodeBase32ToString("MZXW6===")).toBe("foo");
    expect(decodeBase32ToString("MZXW6YQ=")).toBe("foob");
    expect(decodeBase32ToString("MZXW6YTB")).toBe("fooba");
    expect(decodeBase32ToString("MZXW6YTBOI======")).toBe("foobar");
  });

  test("decodes special characters", () => {
    expect(decodeBase32ToString("4OAZHY4CSPRYDK7DQGQ6HANP")).toBe("こんにちは");
  });

  test("throws error on invalid character", () => {
    expect(() => decodeBase32ToString("JBSWY3D@")).toThrow(
      "Invalid base32 character: @",
    );
  });

  test("decodes longer text", () => {
    const encoded =
      "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWO===";
    expect(decodeBase32ToString(encoded)).toBe(
      "The quick brown fox jumps over the lazy dog",
    );
  });
});
