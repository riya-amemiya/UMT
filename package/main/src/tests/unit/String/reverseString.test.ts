import { reverseString } from "@/String/reverseString";
// テストを網羅的に10個書いて
// 絵文字には対応してません
describe("reverseString", () => {
  test("reverseString('abc')", () => {
    expect(reverseString("abc")).toBe("cba");
  });
  test("reverseString('')", () => {
    expect(reverseString("")).toBe("");
  });
  test("reverseString('a')", () => {
    expect(reverseString("a")).toBe("a");
  });
  test("reverseString('ab')", () => {
    expect(reverseString("ab")).toBe("ba");
  });
  test("reverseString('abc')", () => {
    expect(reverseString("abc")).toBe("cba");
  });
  test("reverseString('abcd')", () => {
    expect(reverseString("abcd")).toBe("dcba");
  });
  test("reverseString('abcde')", () => {
    expect(reverseString("abcde")).toBe("edcba");
  });
  test("reverseString('abcdef')", () => {
    expect(reverseString("abcdef")).toBe("fedcba");
  });
  test("reverseString('abcdefg')", () => {
    expect(reverseString("abcdefg")).toBe("gfedcba");
  });
  test("reverseString('abcdefgh')", () => {
    expect(reverseString("abcdefgh")).toBe("hgfedcba");
  });
});
