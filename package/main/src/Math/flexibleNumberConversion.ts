import { isValueNaN } from "@/Validate/isValueNaN";
/**
 * 様々な入力を可能な限り数値に変換する柔軟な関数
 *
 * @param value - 変換する値（任意の型）
 * @returns 変換された数値、または変換できない場合はNaN
 *
 * @description
 * この関数は以下の特徴を持ちます：
 * 1. null、undefined、空文字列は0に変換します
 * 2. すでに数値型の場合はそのまま返します
 * 3. 無限大（Infinity, -Infinity）を適切に処理します
 * 4. 16進数（0x）、8進数（0o）、2進数（0b）の文字列表記に対応します
 * 5. 浮動小数点数の文字列を適切に解析します
 * 6. 数値で始まる文字列から可能な限り数値を抽出します
 * 7. 上記のいずれにも当てはまらない場合はNaNを返します
 *
 * @example
 * flexibleNumberConversion(123)        // 123
 * flexibleNumberConversion("456")      // 456
 * flexibleNumberConversion("78.9")     // 78.9
 * flexibleNumberConversion("3.14e2")   // 314
 * flexibleNumberConversion("0xFF")     // 255
 * flexibleNumberConversion("42px")     // 42
 * flexibleNumberConversion("")         // 0
 * flexibleNumberConversion("not a number") // NaN
 */
export const flexibleNumberConversion = (value: unknown): number => {
  // オブジェクトの場合はNaNを返す
  if (typeof value === "object" && value !== null) {
    return Number.NaN;
  }

  // 特殊なケースの処理
  if (value === null || value === undefined || value === "") {
    return 0;
  }

  // すでに数値型の場合
  if (typeof value === "number" && !isValueNaN(value)) {
    return value;
  }

  // 文字列に変換して処理
  const stringValue = String(value).trim().toLowerCase();

  // 無限大の処理
  if (stringValue === "infinity" || stringValue === "+infinity") {
    return Number.POSITIVE_INFINITY;
  }
  if (stringValue === "-infinity") {
    return Number.NEGATIVE_INFINITY;
  }

  // 特殊な基数表記の処理（16進数、8進数、2進数）
  if (
    stringValue.startsWith("0x") ||
    stringValue.startsWith("0o") ||
    stringValue.startsWith("0b")
  ) {
    return Number(stringValue);
  }

  // 浮動小数点数としての解析
  const floatValue = Number.parseFloat(stringValue);
  if (!isValueNaN(floatValue)) {
    return floatValue;
  }

  // 数値に変換できない場合
  return Number.NaN;
};
