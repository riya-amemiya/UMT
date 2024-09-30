import { convertCurrency } from "@/Math/calculator/convertCurrency";

/**
 * convertCurrency関数のテスト
 */
describe("convertCurrency", () => {
  // ケース 1
  test("conversionRatesが未定義", () => {
    expect(convertCurrency("¥1000")).toBe("¥1000");
  });

  // ケース 2
  test("通貨記号がマッチしない", () => {
    expect(convertCurrency("¥1000", { $: 1.1 })).toBe("¥1000");
  });

  // ケース 3
  test("通貨記号がマッチし、換算レートが数値", () => {
    expect(convertCurrency("¥1000", { "¥": 1.1 })).toBe("1100");
  });

  // ケース 4
  test("通貨記号がマッチし、換算レートが数値でない", () => {
    expect(convertCurrency("¥1000", { "¥": "invalid" })).toBe("¥1000");
  });

  // ケース 5
  test("換算結果がNaN", () => {
    expect(convertCurrency("¥NaN", { "¥": 1.1 })).toBe("¥NaN");
  });
});
