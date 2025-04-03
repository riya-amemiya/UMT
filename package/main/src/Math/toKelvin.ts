import { addition } from "./addition";

/**
 * Converts temperature from Celsius to Kelvin
 * @param {number} celsius Temperature in Celsius
 * @returns {number} Temperature in Kelvin
 * @example toKelvin(26.85); // 300
 * @example toKelvin(0); // 273.15
 * @example toKelvin(-273.15); // 0 (absolute zero)
 */
export const toKelvin = (celsius: number): number => addition(celsius, 273.15);
