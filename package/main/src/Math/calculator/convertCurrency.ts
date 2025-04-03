import { multiplication } from "@/Math/multiplication";
import { isNumber } from "@/Validate/isNumber";

/**
 * Converts currency amounts in a string using currency symbols.
 *
 * @param {string} inputString - String containing a currency amount to convert
 * @param {object} conversionRates - Object mapping currency symbols to conversion rates
 * @returns {string} Converted currency amount as a string, or the original string if conversion is not possible
 * @example convertCurrency("¥100", { "¥": 0.01 }); // "1"
 * @example convertCurrency("$50", { "$": 1.2 }); // "60"
 * @example convertCurrency("€200", { "€": 1.1 }); // "220"
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
