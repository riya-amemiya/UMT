import { decodeBase58ToString } from "@/Crypto/decodeBase58ToString";

describe("decodeBase58ToString", () => {
  test("decodes a simple string", () => {
    expect(decodeBase58ToString("9Ajdvzr")).toBe("Hello");
  });

  test("decodes an empty string", () => {
    expect(decodeBase58ToString("")).toBe("");
  });

  test("decodes single character strings", () => {
    expect(decodeBase58ToString("2g")).toBe("a");
    expect(decodeBase58ToString("2h")).toBe("b");
    expect(decodeBase58ToString("2i")).toBe("c");
  });

  test("decodes special characters", () => {
    expect(decodeBase58ToString("7NAasPYBzpyEe5hmwr1KL")).toBe("こんにちは");
  });

  test("throws error on invalid character", () => {
    expect(() => decodeBase58ToString("9Ajdvz0")).toThrow(
      "Invalid base58 character: 0",
    );
    expect(() => decodeBase58ToString("9AjdvzO")).toThrow(
      "Invalid base58 character: O",
    );
  });

  test("decodes longer text", () => {
    const encoded =
      "7DdiPPYtxLjCD3wA1po2rvZHTDYjkZYiEtazrfiwJcwnKCizhGFhBGHeRdx";
    expect(decodeBase58ToString(encoded)).toBe(
      "The quick brown fox jumps over the lazy dog",
    );
  });
});
