import { subtract } from "./subtract";

/**
 * Converts temperature from Kelvin to Celsius
 * @param {number} kelvin Temperature in Kelvin
 * @returns {number} Temperature in Celsius
 * @example toCelsius(300); // 26.85
 * @example toCelsius(273.15); // 0
 * @example toCelsius(0); // -273.15 (absolute zero)
 */
export const toCelsius = (kelvin: number): number => subtract(kelvin, 273.15);
