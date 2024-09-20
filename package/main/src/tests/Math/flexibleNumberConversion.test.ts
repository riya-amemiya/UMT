import { flexibleNumberConversion } from "@/Math/flexibleNumberConversion";

describe("flexibleNumberConversion", () => {
  test("数値をそのまま返す", () => {
    expect(flexibleNumberConversion(123)).toBe(123);
    expect(flexibleNumberConversion(-456)).toBe(-456);
    expect(flexibleNumberConversion(0)).toBe(0);
  });

  test("数値の文字列を適切に変換する", () => {
    expect(flexibleNumberConversion("789")).toBe(789);
    expect(flexibleNumberConversion("-101")).toBe(-101);
    expect(flexibleNumberConversion("3.14")).toBe(3.14);
  });

  test("指数表記を正しく解釈する", () => {
    expect(flexibleNumberConversion("3.14e2")).toBe(314);
    expect(flexibleNumberConversion("1e-2")).toBe(0.01);
  });

  test("特殊な基数表記を正しく解釈する", () => {
    expect(flexibleNumberConversion("0xFF")).toBe(255);
    expect(flexibleNumberConversion("0b1010")).toBe(10);
    expect(flexibleNumberConversion("0o777")).toBe(511);
  });

  test("Infinityを適切に処理する", () => {
    expect(flexibleNumberConversion("Infinity")).toBe(Infinity);
    expect(flexibleNumberConversion("-Infinity")).toBe(-Infinity);
  });

  test("数値で始まる文字列から数値を抽出する", () => {
    expect(flexibleNumberConversion("42px")).toBe(42);
    expect(flexibleNumberConversion("-42px")).toBe(-42);
    expect(flexibleNumberConversion("3.14meters")).toBe(3.14);
  });

  test("特殊な入力を0として扱う", () => {
    expect(flexibleNumberConversion("")).toBe(0);
    expect(flexibleNumberConversion(null)).toBe(0);
    expect(flexibleNumberConversion(undefined)).toBe(0);
  });

  test("無効な入力に対してNaNを返す", () => {
    expect(flexibleNumberConversion("not a number")).toBe(NaN);
    expect(flexibleNumberConversion("abc")).toBe(NaN);
    expect(flexibleNumberConversion({})).toBe(NaN);
    expect(flexibleNumberConversion([1, 2, 3])).toBe(NaN);
  });

  test("数値の前後にスペースがある文字列を適切に変換する", () => {
    expect(flexibleNumberConversion(" 123 ")).toBe(123);
    expect(flexibleNumberConversion("  -456px ")).toBe(-456);
  });

  test("プラス符号のみの文字列を処理する", () => {
    expect(flexibleNumberConversion("+")).toBe(NaN);
    expect(flexibleNumberConversion("-")).toBe(NaN);
  });

  test("無効な指数表記を持つ文字列を処理する", () => {
    expect(flexibleNumberConversion("1e")).toBe(1);
    expect(flexibleNumberConversion("e10")).toBe(NaN);
  });

  test("小数点のみの文字列を処理する", () => {
    expect(flexibleNumberConversion(".")).toBe(NaN);
    expect(flexibleNumberConversion("-.")).toBe(NaN);
  });

  test("数値で始まらない文字列を処理する", () => {
    expect(flexibleNumberConversion("px42")).toBe(NaN);
    expect(flexibleNumberConversion("meter3.14")).toBe(NaN);
  });

  test("大きな数値と小さな数値を適切に処理する", () => {
    expect(flexibleNumberConversion(Number.MAX_VALUE)).toBe(Number.MAX_VALUE);
    expect(flexibleNumberConversion(Number.MIN_VALUE)).toBe(Number.MIN_VALUE);
  });

  test("文字列中の複数の数値を処理する", () => {
    expect(flexibleNumberConversion("42 and 24")).toBe(42);
    expect(flexibleNumberConversion("3.14 and 2.71")).toBe(3.14);
  });
});
