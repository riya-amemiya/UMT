import { addition } from "./addition";

/**
 * 摂氏をケルビンに変換します。
 * @param celsius
 * @returns number
 * @example toKelvin(26.85); // 300
 */
export const toKelvin = (celsius: number): number => addition(celsius, 273.15);
