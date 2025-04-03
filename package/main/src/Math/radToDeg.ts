import { division } from "./division";

/**
 * Converts radians to degrees
 * @param  {number} x Angle in radians
 * @returns {number} Angle in degrees
 * @example radToDeg(Math.PI); // 180
 * @description
 * Uses the formula: degrees = radians * (180/π)
 */
export const radToDeg = (x: number) => division(x, division(Math.PI, 180));
