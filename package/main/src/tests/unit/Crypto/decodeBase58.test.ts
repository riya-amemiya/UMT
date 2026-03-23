import { decodeBase58 } from "@/Crypto/decodeBase58";

describe("decodeBase58", () => {
  test("decodes a simple string", () => {
    const result = decodeBase58("9Ajdvzr");
    expect(new TextDecoder().decode(result)).toBe("Hello");
  });

  test("decodes an empty string", () => {
    const result = decodeBase58("");
    expect(result.length).toBe(0);
  });

  test("decodes single character strings", () => {
    expect(new TextDecoder().decode(decodeBase58("2g"))).toBe("a");
    expect(new TextDecoder().decode(decodeBase58("2h"))).toBe("b");
    expect(new TextDecoder().decode(decodeBase58("2i"))).toBe("c");
  });

  test("handles leading ones (zeros)", () => {
    const result = decodeBase58("119Ajdvzr");
    expect(result[0]).toBe(0);
    expect(result[1]).toBe(0);
    expect(new TextDecoder().decode(result.slice(2))).toBe("Hello");
  });

  test("decodes longer text", () => {
    const encoded =
      "7DdiPPYtxLjCD3wA1po2rvZHTDYjkZYiEtazrfiwJcwnKCizhGFhBGHeRdx";
    const result = new TextDecoder().decode(decodeBase58(encoded));
    expect(result).toBe("The quick brown fox jumps over the lazy dog");
  });

  test("decodes binary data", () => {
    const result = decodeBase58("Vt9aq46");
    expect(Array.from(result)).toEqual([255, 254, 253, 252, 251]);
  });

  test("treats invalid characters as 0", () => {
    // '0' is not in the Base58 alphabet, so it should fall back to 0 via ?? 0
    const resultWithInvalid = decodeBase58("0");
    // Invalid char maps to index 0, bigNumber stays 0, no bytes produced,
    // and '0' is not '1' so no leading zeros are added
    expect(Array.from(resultWithInvalid)).toEqual([]);
  });
});
