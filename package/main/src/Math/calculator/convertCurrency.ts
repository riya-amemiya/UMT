import { isNumber } from "../isNumber";
import { multiplication } from "../multiplication";

/**
 * 文字列内の通貨記号を利用して通貨を換算する。
 *
 * @param inputString - 換算する通貨額を含む文字列
 * @param conversionRates - 通貨記号と換算レートのオブジェクト
 * @returns 換算後の通貨額を文字列で返す。換算できない場合は元の文字列を返す。
 */
export const convertCurrency = <T extends object>(
  inputString: string,
  conversionRates?: T,
) => {
  // biome-ignore lint/style/useBlockStatements: <explanation>
  if (!conversionRates) return inputString;

  for (const currencySymbol in conversionRates) {
    if (inputString.startsWith(currencySymbol)) {
      const amountStr = inputString.slice(currencySymbol.length);
      const rate = conversionRates[currencySymbol];

      if (isNumber(rate)) {
        const amount = Number(amountStr);
        const convertedAmount = multiplication(amount, Number(rate));

        return Number.isNaN(convertedAmount)
          ? inputString
          : String(convertedAmount);
      }
    }
  }
  return inputString;
};
