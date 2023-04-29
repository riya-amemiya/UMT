import { subtract } from "./subtract";

/**
 * Converts a temperature from Kelvin to Celsius.
 * @param kelvin The temperature in Kelvin.
 * @returns The temperature in Celsius.
 */
export const toCelsius = (kelvin: number) => subtract(kelvin, 273.15);
