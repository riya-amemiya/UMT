import { isNumber } from "@/Validate/isNumber";
describe("isNumber", () => {
  // 引数が数値であるケース
  test("should return true when a number is provided", () => {
    expect(isNumber(5)).toBe(true);
  });

  // 引数が文字列であるが、数値に変換可能なケース
  test("should return true when a string of a number is provided (loose mode)", () => {
    expect(isNumber("5", true)).toBe(true);
  });

  // 引数が文字列であるが、数値に変換不可能なケース
  test("should return false when a non-numeric string is provided", () => {
    expect(isNumber("abc", true)).toBe(false);
  });

  // 引数がnullであるケース
  test("should return false when null is provided", () => {
    expect(isNumber(null)).toBe(false);
  });

  // 引数がbooleanであるケース
  test("should return false when a boolean is provided", () => {
    expect(isNumber(true)).toBe(false);
  });

  // 引数が文字列であるが、looseモードがオフのケース
  test("should return false when a string is provided and loose mode is off", () => {
    expect(isNumber("5", false)).toBe(false);
  });

  // 空文字列が引数として渡されたケース
  test("should return false when an empty string is provided", () => {
    expect(isNumber("")).toBe(true);
  });
});
