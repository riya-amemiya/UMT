import { reverseString } from "@/String/reverseString";
describe("reverseString", () => {
  test("should reverse a three-character string", () => {
    expect(reverseString("abc")).toBe("cba");
  });
  test("should handle empty string", () => {
    expect(reverseString("")).toBe("");
  });
  test("should handle single character", () => {
    expect(reverseString("a")).toBe("a");
  });
  test("should reverse a two-character string", () => {
    expect(reverseString("ab")).toBe("ba");
  });
  test("should reverse a three-character string (second test)", () => {
    expect(reverseString("abc")).toBe("cba");
  });
  test("should reverse a four-character string", () => {
    expect(reverseString("abcd")).toBe("dcba");
  });
  test("should reverse a five-character string", () => {
    expect(reverseString("abcde")).toBe("edcba");
  });
  test("should reverse a six-character string", () => {
    expect(reverseString("abcdef")).toBe("fedcba");
  });
  test("should reverse a seven-character string", () => {
    expect(reverseString("abcdefg")).toBe("gfedcba");
  });
  test("should reverse an eight-character string", () => {
    expect(reverseString("abcdefgh")).toBe("hgfedcba");
  });

  test("should handle whitespace", () => {
    expect(reverseString("hello world")).toBe("dlrow olleh");
    expect(reverseString("  ")).toBe("  ");
  });

  test("should handle special characters", () => {
    expect(reverseString("!@#$%")).toBe("%$#@!");
    expect(reverseString("a-b-c")).toBe("c-b-a");
  });

  test("should handle numbers in string", () => {
    expect(reverseString("12345")).toBe("54321");
    expect(reverseString("a1b2c3")).toBe("3c2b1a");
  });

  test("should handle unicode characters", () => {
    expect(reverseString("あいう")).toBe("ういあ");
    expect(reverseString("日本語")).toBe("語本日");
  });
});
