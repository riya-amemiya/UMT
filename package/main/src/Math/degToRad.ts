import { division } from "./division";
import { multiplication } from "./multiplication";

const PI_OVER_180 = division(Math.PI, 180);

/**
Converts degrees to radians
@param  {number} x Angle in degrees
@returns number Angle in radians
@example degToRad(180); // 3.141592653589793
*/
export const degToRad = (x: number) => {
  return multiplication(x, PI_OVER_180);
};
