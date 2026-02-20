import { decodeBase32 } from "@/Crypto/decodeBase32";

describe("decodeBase32", () => {
  test("decodes a simple string", () => {
    const result = decodeBase32("JBSWY3DP");
    expect(new TextDecoder().decode(result)).toBe("Hello");
  });

  test("decodes an empty string", () => {
    const result = decodeBase32("");
    expect(result.length).toBe(0);
  });

  test("decodes strings with padding", () => {
    expect(new TextDecoder().decode(decodeBase32("MY======"))).toBe("f");
    expect(new TextDecoder().decode(decodeBase32("MZXQ===="))).toBe("fo");
    expect(new TextDecoder().decode(decodeBase32("MZXW6==="))).toBe("foo");
    expect(new TextDecoder().decode(decodeBase32("MZXW6YQ="))).toBe("foob");
    expect(new TextDecoder().decode(decodeBase32("MZXW6YTB"))).toBe("fooba");
    expect(new TextDecoder().decode(decodeBase32("MZXW6YTBOI======"))).toBe(
      "foobar",
    );
  });

  test("decodes longer text", () => {
    const encoded =
      "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWO===";
    const result = new TextDecoder().decode(decodeBase32(encoded));
    expect(result).toBe("The quick brown fox jumps over the lazy dog");
  });

  test("handles Base32 without padding", () => {
    expect(new TextDecoder().decode(decodeBase32("JBSWY3DP"))).toBe("Hello");
    expect(new TextDecoder().decode(decodeBase32("MZXW6YTB"))).toBe("fooba");
  });
});
