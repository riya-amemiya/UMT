import { calculator } from ".";

/**
 * Initializes the calculator.
 * @param {object} exchange - current exchange rate
 * @return {Function} - calculator
 */
export const calculatorInitialization = <T extends object>(
	exchange: T,
): ((x: string) => string) => {
	/**
	 * @param {string} x - amount of money
	 * @return {string} - converted amount of money
	 */
	return (x: string): string => calculator(x, exchange);
};
