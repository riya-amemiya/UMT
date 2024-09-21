import { flexibleNumberConversion } from "@/Math/flexibleNumberConversion";

describe("flexibleNumberConversion", () => {
  // 基本的な数値入力
  describe("基本的な数値入力", () => {
    test("正の整数をそのまま返す", () => {
      expect(flexibleNumberConversion(123)).toBe(123);
      expect(flexibleNumberConversion(456)).toBe(456);
      expect(flexibleNumberConversion(789)).toBe(789);
    });

    test("負の整数をそのまま返す", () => {
      expect(flexibleNumberConversion(-123)).toBe(-123);
      expect(flexibleNumberConversion(-456)).toBe(-456);
      expect(flexibleNumberConversion(-789)).toBe(-789);
    });

    test("ゼロをそのまま返す", () => {
      expect(flexibleNumberConversion(0)).toBe(0);
      expect(flexibleNumberConversion(-0)).toBe(-0);
    });
  });

  // 文字列としての数値入力
  describe("文字列としての数値入力", () => {
    test("正の整数の文字列を適切に変換する", () => {
      expect(flexibleNumberConversion("123")).toBe(123);
      expect(flexibleNumberConversion("456")).toBe(456);
      expect(flexibleNumberConversion("789")).toBe(789);
    });

    test("負の整数の文字列を適切に変換する", () => {
      expect(flexibleNumberConversion("-123")).toBe(-123);
      expect(flexibleNumberConversion("-456")).toBe(-456);
      expect(flexibleNumberConversion("-789")).toBe(-789);
    });

    test("小数の文字列を適切に変換する", () => {
      expect(flexibleNumberConversion("3.14")).toBe(3.14);
      expect(flexibleNumberConversion("-2.718")).toBe(-2.718);
      expect(flexibleNumberConversion("0.001")).toBe(0.001);
    });

    test("指数表記の文字列を正しく解釈する", () => {
      expect(flexibleNumberConversion("1e3")).toBe(1000);
      expect(flexibleNumberConversion("-2.5e-3")).toBe(-0.0025);
      expect(flexibleNumberConversion("6.022e23")).toBe(6.022e23);
    });

    test("特殊な基数表記の文字列を正しく解釈する", () => {
      expect(flexibleNumberConversion("0x1A")).toBe(26);
      expect(flexibleNumberConversion("0b1011")).toBe(11);
      expect(flexibleNumberConversion("0o77")).toBe(63);
    });

    test("数値の前後にスペースがある文字列を適切に変換する", () => {
      expect(flexibleNumberConversion(" 123 ")).toBe(123);
      expect(flexibleNumberConversion("   -456")).toBe(-456);
      expect(flexibleNumberConversion("789   ")).toBe(789);
    });

    test("プラス符号のみの文字列を処理する", () => {
      expect(flexibleNumberConversion("+")).toBe(NaN);
      expect(flexibleNumberConversion("+123")).toBe(123);
      expect(flexibleNumberConversion("+0")).toBe(0);
    });

    test("無効な指数表記を持つ文字列を処理する", () => {
      expect(flexibleNumberConversion("1e")).toBe(1);
      expect(flexibleNumberConversion("e10")).toBe(NaN);
      expect(flexibleNumberConversion("2e+")).toBe(2);
    });

    test("小数点のみの文字列を処理する", () => {
      expect(flexibleNumberConversion(".")).toBe(NaN);
      expect(flexibleNumberConversion("-.")).toBe(NaN);
      expect(flexibleNumberConversion(".e1")).toBe(NaN);
    });

    test("数値で始まらない文字列を処理する", () => {
      expect(flexibleNumberConversion("px42")).toBe(NaN);
      expect(flexibleNumberConversion("meter3.14")).toBe(NaN);
      expect(flexibleNumberConversion("abc123")).toBe(NaN);
    });
  });

  // 特殊な数値入力
  describe("特殊な数値入力", () => {
    test("Infinityを適切に処理する", () => {
      expect(flexibleNumberConversion("Infinity")).toBe(Infinity);
      expect(flexibleNumberConversion("-Infinity")).toBe(-Infinity);
      expect(flexibleNumberConversion(Infinity)).toBe(Infinity);
      expect(flexibleNumberConversion(-Infinity)).toBe(-Infinity);
    });

    test("NaNの入力を処理する", () => {
      expect(flexibleNumberConversion(NaN)).toBe(NaN);
      expect(flexibleNumberConversion("NaN")).toBe(NaN);
    });

    test("非常に大きな数値を適切に処理する", () => {
      expect(flexibleNumberConversion(Number.MAX_SAFE_INTEGER)).toBe(
        Number.MAX_SAFE_INTEGER,
      );
      expect(flexibleNumberConversion("9007199254740991")).toBe(
        9007199254740991,
      );
      expect(flexibleNumberConversion("1e308")).toBe(1e308);
    });

    test("非常に小さな数値を適切に処理する", () => {
      expect(flexibleNumberConversion(Number.MIN_VALUE)).toBe(Number.MIN_VALUE);
      expect(flexibleNumberConversion("5e-324")).toBe(5e-324);
      expect(flexibleNumberConversion("-1e-308")).toBe(-1e-308);
    });
  });

  // 無効または特殊な入力
  describe("無効または特殊な入力", () => {
    test("空文字列、null、undefinedを0として扱う", () => {
      expect(flexibleNumberConversion("")).toBe(0);
      expect(flexibleNumberConversion(null)).toBe(0);
      expect(flexibleNumberConversion(undefined)).toBe(0);
    });

    test("無効な文字列をNaNとして返す", () => {
      expect(flexibleNumberConversion("not a number")).toBe(NaN);
      expect(flexibleNumberConversion("abc")).toBe(NaN);
      expect(flexibleNumberConversion("abc123")).toBe(NaN);
    });

    test("オブジェクトや配列をNaNとして返す", () => {
      expect(flexibleNumberConversion({})).toBe(NaN);
      expect(flexibleNumberConversion({ key: "value" })).toBe(NaN);
      expect(flexibleNumberConversion([])).toBe(NaN);
      expect(flexibleNumberConversion([1, 2, 3])).toBe(NaN);
    });

    test("関数をNaNとして返す", () => {
      const func = () => {};
      expect(flexibleNumberConversion(func)).toBe(NaN);
      expect(flexibleNumberConversion(function () {})).toBe(NaN);
      expect(flexibleNumberConversion(() => {})).toBe(NaN);
    });

    test("特殊文字を含む文字列をNaNとして返す", () => {
      expect(flexibleNumberConversion("@123")).toBe(NaN);
      expect(flexibleNumberConversion("!@#$%")).toBe(NaN);
    });
  });

  // 複雑なケース
  describe("複雑なケース", () => {
    test("数値と文字が混在する文字列の先頭を数値として抽出する", () => {
      expect(flexibleNumberConversion("42px")).toBe(42);
      expect(flexibleNumberConversion("-42px")).toBe(-42);
      expect(flexibleNumberConversion("3.14meters")).toBe(3.14);
      expect(flexibleNumberConversion("-3.14meters")).toBe(-3.14);
      expect(flexibleNumberConversion("1e10meters")).toBe(1e10);
      expect(flexibleNumberConversion("-1e10meters")).toBe(-1e10);
    });

    test("内部にスペースが含まれる文字列を適切に処理する", () => {
      expect(flexibleNumberConversion(" 123 456 ")).toBe(123);
      expect(flexibleNumberConversion(" -456px ")).toBe(-456);
      expect(flexibleNumberConversion("3.14 meters")).toBe(3.14);
      expect(flexibleNumberConversion("1e2 meters")).toBe(100);
    });

    test("先頭にゼロが付く数値を適切に処理する", () => {
      expect(flexibleNumberConversion("0123")).toBe(123);
      expect(flexibleNumberConversion("-0456")).toBe(-456);
      expect(flexibleNumberConversion("000789")).toBe(789);
    });

    test("先頭にプラス符号が付く数値を適切に処理する", () => {
      expect(flexibleNumberConversion("+123")).toBe(123);
      expect(flexibleNumberConversion("+0")).toBe(0);
      expect(flexibleNumberConversion("+3.14")).toBe(3.14);
      expect(flexibleNumberConversion("+1e10")).toBe(1e10);
    });
  });

  // パフォーマンスとストレステスト
  describe("パフォーマンスとストレステスト", () => {
    test("大量の連続した有効な入力を処理する", () => {
      for (let i = 0; i < 1000; i++) {
        expect(flexibleNumberConversion(i.toString())).toBe(i);
      }
    });

    test("大量の連続した無効な入力を処理する", () => {
      for (let i = 0; i < 1000; i++) {
        expect(flexibleNumberConversion(`invalid${i}`)).toBe(NaN);
      }
    });
  });

  // 境界値テスト
  describe("境界値テスト", () => {
    test("Number.MAX_VALUEを処理する", () => {
      expect(flexibleNumberConversion(Number.MAX_VALUE)).toBe(Number.MAX_VALUE);
      expect(flexibleNumberConversion("1.7976931348623157e+308")).toBe(
        Number.MAX_VALUE,
      );
    });

    test("Number.MIN_VALUEを処理する", () => {
      expect(flexibleNumberConversion(Number.MIN_VALUE)).toBe(Number.MIN_VALUE);
      expect(flexibleNumberConversion("5e-324")).toBe(Number.MIN_VALUE);
    });

    test("Number.MAX_SAFE_INTEGERを処理する", () => {
      expect(flexibleNumberConversion(Number.MAX_SAFE_INTEGER)).toBe(
        Number.MAX_SAFE_INTEGER,
      );
      expect(flexibleNumberConversion("9007199254740991")).toBe(
        Number.MAX_SAFE_INTEGER,
      );
    });

    test("Number.MIN_SAFE_INTEGERを処理する", () => {
      expect(flexibleNumberConversion(Number.MIN_SAFE_INTEGER)).toBe(
        Number.MIN_SAFE_INTEGER,
      );
      expect(flexibleNumberConversion("-9007199254740991")).toBe(
        Number.MIN_SAFE_INTEGER,
      );
    });
  });
});
