import { division } from "./division";
import { multiplication } from "./multiplication";
/**
 * Converts degrees to radians
 * @param  {number} x Angle in degrees
 * @returns number Angle in radians
 * @example degToRad(180); // 3.141592653589793
 */
export const degToRad = (x: number) => {
  return multiplication(x, division(Math.PI, 180));
};
