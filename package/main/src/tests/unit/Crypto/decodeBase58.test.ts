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

  test("throws error on invalid character", () => {
    expect(() => decodeBase58("9Ajdvz0")).toThrow(
      "Invalid base58 character: 0",
    );
    expect(() => decodeBase58("9AjdvzO")).toThrow(
      "Invalid base58 character: O",
    );
    expect(() => decodeBase58("9AjdvzI")).toThrow(
      "Invalid base58 character: I",
    );
    expect(() => decodeBase58("9Ajdvzl")).toThrow(
      "Invalid base58 character: l",
    );
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
});
