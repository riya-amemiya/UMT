import { calculator } from "./calculator";

/**
 * Initializes a calculator function with exchange rates
 * @param {object} exchange - Exchange rates object
 * @return {Function} - Configured calculator function
 * @example calculatorInitialization({ $: 100 })("$1"); // "100"
 * @example calculatorInitialization({ EUR: 1.2 })("EUR5 + 10"); // "16"
 */
export const calculatorInitialization = <
  T extends { [key: string]: string | number },
>(
  exchange: T,
): ((x: string) => string) => {
  /**
   * @param {string} x - Expression to calculate
   * @return {string} - Calculation result
   */
  return (x: string): string => calculator(x, exchange);
};
