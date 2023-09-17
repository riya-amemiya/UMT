import { convertCurrency } from "@/Math/calculator/convertCurrency";

/**
 * convertCurrency関数のテスト
 */
test("convertCurrency", () => {
  // "$1" を 100 倍して "100" になることを確認
  expect(convertCurrency("$1", { $: 100 })).toBe("100");

  // 未知の通貨記号 "a$" が入力された場合、元の文字列 "a$1" を返すことを確認
  expect(convertCurrency("a$1", { $: 100 })).toBe("a$1");

  // "a$1" が 100 倍して "100" になることを確認
  expect(convertCurrency("a$1", { a$: 100 })).toBe("100");

  // 換算レートが指定されていない場合、元の文字列 "$1" を返すことを確認
  expect(convertCurrency("$1")).toBe("$1");

  // 換算レートが数値でない場合、元の文字列 "$1" を返すことを確認
  expect(convertCurrency("$1", { $: "abc" })).toBe("$1");

  // 換算レートが文字列の数値の場合、"$1" を 100 倍して "100" になることを確認
  expect(convertCurrency("$1", { $: "100" })).toBe("100");
});
