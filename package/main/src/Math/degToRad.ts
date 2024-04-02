import { division } from "./division";
import { multiplication } from "./multiplication";
/**
 * 角度をラジアンに変換
 * @param  {number} x
 * @returns number
 * @example degToRad(180); // 3.141592653589793
 */
export const degToRad = (x: number) => {
  return multiplication(x, division(Math.PI, 180));
};
