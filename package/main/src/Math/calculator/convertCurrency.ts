import { multiplication } from "@/Math/multiplication";
import { isNumber } from "@/Validate/isNumber";

/**
 * 文字列内の通貨記号を利用して通貨を換算する。
 *
 * @param inputString - 換算する通貨額を含む文字列
 * @param conversionRates - 通貨記号と換算レートのオブジェクト
 * @returns 換算後の通貨額を文字列で返す。換算できない場合は元の文字列を返す。
 * @example convertCurrency("¥100", { "¥": 0.01 }); // "1"
 */
export const convertCurrency = <
  T extends {
    [key: string]: number | string;
  },
>(
  inputString: string,
  conversionRates?: T,
) => {
  if (!conversionRates) {
    return inputString;
  }

  for (const currencySymbol in conversionRates) {
    if (inputString.startsWith(currencySymbol)) {
      const amountString = inputString.slice(currencySymbol.length);
      const rate: string | number = conversionRates[currencySymbol];

      if (isNumber(rate)) {
        const amount = Number(amountString);
        const convertedAmount = multiplication(amount, Number(rate));
        return Number.isNaN(convertedAmount)
          ? inputString
          : String(convertedAmount);
      }
    }
  }
  return inputString;
};
