import { encodeBase58 } from "@/Crypto/encodeBase58";

describe("encodeBase58", () => {
  test("encodes a simple string", () => {
    expect(encodeBase58("Hello")).toBe("9Ajdvzr");
  });

  test("encodes an empty string", () => {
    expect(encodeBase58("")).toBe("");
  });

  test("encodes single characters", () => {
    expect(encodeBase58("a")).toBe("2g");
    expect(encodeBase58("b")).toBe("2h");
    expect(encodeBase58("c")).toBe("2i");
  });

  test("encodes a Uint8Array", () => {
    const bytes = new Uint8Array([72, 101, 108, 108, 111]); // "Hello"
    expect(encodeBase58(bytes)).toBe("9Ajdvzr");
  });

  test("handles leading zeros", () => {
    const bytes = new Uint8Array([0, 0, 72, 101, 108, 108, 111]);
    expect(encodeBase58(bytes)).toBe("119Ajdvzr");
  });

  test("encodes longer text", () => {
    const text = "The quick brown fox jumps over the lazy dog";
    expect(encodeBase58(text)).toBe(
      "7DdiPPYtxLjCD3wA1po2rvZHTDYjkZYiEtazrfiwJcwnKCizhGFhBGHeRdx",
    );
  });

  test("encodes special characters", () => {
    expect(encodeBase58("こんにちは")).toBe("7NAasPYBzpyEe5hmwr1KL");
  });

  test("encodes binary data", () => {
    const bytes = new Uint8Array([255, 254, 253, 252, 251]);
    expect(encodeBase58(bytes)).toBe("Vt9aq46");
  });
});
