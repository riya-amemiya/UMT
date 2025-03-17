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
});
