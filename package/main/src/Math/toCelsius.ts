import { subtract } from "./subtract";

/**
 * ケルビンを摂氏に変換する
 * @param kelvin
 * @returns number
 * @example toCelsius(300); // 26.85
 */
export const toCelsius = (kelvin: number) => subtract(kelvin, 273.15);
