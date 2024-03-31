import { division } from "./division";

/**
 * ラジアンを角度に変換
 * @param  {number} x
 * @returns number
 * @example radToDeg(Math.PI); // 180
 */
export const radToDeg = (x: number) => division(x, division(Math.PI, 180));
